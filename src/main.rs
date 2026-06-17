use colored::*;
use dialoguer::MultiSelect;
use regex::Regex;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("{}", "🔍 Searching for branches tagged as [gone]...".cyan());

    match refresh_branches() {
        Ok(_) => println!("{}", "🔄️ Branches refreshed successfully.".green()),
        Err(e) => {
            eprintln!("{}", format!("❌ Failed to refresh branches: {}", e).red());
            return;
        }
    }

    let current_branch = match get_current_branch() {
        Ok(b) => b,
        Err(e) => {
            eprintln!(
                "{}",
                format!("❌ Failed do get current branch: {}", e).red()
            );
            return;
        }
    };

    println!(
        "{}",
        format!("📍 Current branch: {} will not be deleted", current_branch).yellow()
    );

    let gone_branches = match get_gone_branches() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}", format!("❌ Failed to get gone branches: {}", e).red());
            return;
        }
    };

    println!(
        "{}",
        format!("🔍 Found {} branches tagged as [gone]", gone_branches.len()).cyan()
    );

    if gone_branches.is_empty() {
        return;
    }

    print_gone_branches(&gone_branches, &current_branch);

    let selected_branches = match select_branches_to_delete(&gone_branches, &current_branch) {
        Ok(branches) => branches,
        Err(e) => {
            eprintln!("{}", format!("❌ Selection failed: {}", e).red());
            return;
        }
    };

    if selected_branches.is_empty() {
        println!("{}", "⚠️ No branches selected for deletion.".yellow());
        return;
    }

    if !ask_confirmation() {
        println!("{}", "❌ Operation cancelled by user.".red());
        return;
    }

    let summary = delete_branches(selected_branches, &current_branch);
    print_summary(&summary);

    println!("\nPress any key to exit...");
    let _ = io::stdin().read_line(&mut String::new());
}

fn get_current_branch() -> Result<String, String> {
    let out = run_git(&["symbolic-ref", "--short", "HEAD"])?;
    Ok(out.trim().to_string())
}

fn refresh_branches() -> Result<String, String> {
    let out = run_git(&["fetch", "--prune"])?;
    Ok(out.trim().to_string())
}

fn run_git(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to exec command: git {:?}: {}", args, e))?;

    if !output.status.success() {
        return Err(format!("get {:?} returned code {}", args, output.status));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}

fn get_gone_branches() -> Result<Vec<String>, String> {
    let output = run_git(&["branch", "-v"])?;
    let re = Regex::new(r"^\s*(\*?)\s*(\S+)\s+\S+\s+\[gone]").unwrap();

    let mut gone = Vec::new();

    for line in output.lines() {
        if !line.contains("[gone]") {
            continue;
        }

        if let Some(caps) = re.captures(line) {
            if let Some(m) = caps.get(2) {
                gone.push(m.as_str().to_string());
            }
        }
    }

    Ok(gone)
}

fn ask_confirmation() -> bool {
    print!("⚠️ Do you want to delete these branches? (Y/N) ");
    io::stdout().flush().ok();

    let mut input = String::new();

    if io::stdin().read_line(&mut input).is_err() {
        return false;
    }

    match input.trim().to_lowercase().as_str() {
        "s" | "sim" | "y" | "yes" => true,
        _ => false,
    }
}

fn print_gone_branches(gone: &[String], current: &str) {
    println!();
    println!("{}", "📋 Branches marked as [gone]:".cyan());

    for (index, branch) in gone.iter().enumerate() {
        if branch == current {
            println!(
                "{}",
                format!(
                    "  {}. {} (current branch, will be skipped)",
                    index + 1,
                    branch
                )
                .yellow()
            );
        } else {
            println!("{}", format!("  {}. {}", index + 1, branch).white());
        }
    }
}

fn select_branches_to_delete(gone: &[String], current: &str) -> Result<Vec<String>, String> {
    let eligible: Vec<String> = gone
        .iter()
        .filter(|branch| branch.as_str() != current)
        .cloned()
        .collect();

    println!();
    println!(
        "{}",
        "✅ All eligible branches are selected for deletion by default.".green()
    );
    println!(
        "{}",
        "Use Space to toggle a branch, Enter to confirm, and arrows to move.".white()
    );
    println!();

    let defaults = vec![true; eligible.len()];
    let selected_indexes = MultiSelect::new()
        .with_prompt("Select branches to delete")
        .items(&eligible)
        .defaults(&defaults)
        .interact()
        .map_err(|e| format!("interactive prompt failed: {}", e))?;

    Ok(selected_indexes
        .into_iter()
        .map(|index| eligible[index].clone())
        .collect())
}

struct Summary {
    total: usize,
    deleted: Vec<String>,
    skipped: Vec<String>,
    failed: Vec<String>,
}

fn delete_branches(gone: Vec<String>, current: &str) -> Summary {
    let mut summary = Summary {
        total: gone.len(),
        deleted: Vec::new(),
        skipped: Vec::new(),
        failed: Vec::new(),
    };

    for branch in gone {
        if branch == current {
            println!(
                "{}",
                format!("⚠️ Skipping current branch: {}", branch).yellow()
            );
            summary.skipped.push(branch);
            continue;
        }

        let output = Command::new("git").args(["branch", "-D", &branch]).output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    println!("{}", format!("✅ Deleted branch: {}", branch).green());
                    summary.deleted.push(branch);
                } else {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    println!(
                        "{}",
                        format!("❌ Failed to delete branch {}: {}", branch, stderr.trim()).red()
                    );
                    summary.failed.push(branch);
                }
            }
            Err(e) => {
                println!(
                    "{}",
                    format!("❌ Failed to delete branch {}: {}", branch, e).red()
                );
                summary.failed.push(branch);
            }
        }
    }

    summary
}
fn print_summary(summary: &Summary) {
    println!();
    println!("{}", "📊 Operation details:".cyan());
    println!(
        "{} {}",
        "+ Count of branches [gone]:".white(),
        summary.total
    );
    println!(
        "{} {}",
        "+ Count of deleted branches:".green(),
        summary.deleted.len()
    );
    println!(
        "{} {}",
        "+ Skipped branches:".yellow(),
        summary.skipped.len()
    );
    println!("{} {}", "+ Failed to delete:".red(), summary.failed.len());

    if !summary.deleted.is_empty() {
        println!();
        println!("{}", "✅ Deleted branches:".green());
        for b in &summary.deleted {
            println!("{}", format!("  - {}", b).green());
        }
    }

    if !summary.skipped.is_empty() {
        println!();
        println!("{}", "⚠️ Skipped branches (current):".yellow());
        for b in &summary.skipped {
            println!("{}", format!("  - {}", b).yellow());
        }
    }

    if !summary.failed.is_empty() {
        println!();
        println!("{}", "❌ Failed to delete:".red());
        for b in &summary.failed {
            println!("{}", format!("  - {}", b).red());
        }
    }
}
