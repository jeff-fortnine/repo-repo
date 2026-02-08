use clap::Parser;
use colored::Colorize;
use std::env::set_current_dir;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

// TODO implement interactive local branch maintenance
// TODO improve error output

/// A simple program for performing maintenance on Git repositories
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The root directory to search for repositories (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Search for repositories recursively
    #[arg(short, long)]
    recursive: bool,
}

fn main() {
    let args = Args::parse();
    let repos = find_git_repos(&args.path, args.recursive);

    if repos.is_empty() {
        println!("{}", "No Git repositories found.".red());
        return;
    }

    for repo in repos {
        println!(
            "{} {}",
            "Maintaining".blue(),
            repo.display().to_string().bold()
        );
        let repo_path = repo.to_str().unwrap();
        do_maintenance_on(repo_path);
    }
}

fn find_git_repos(root: &PathBuf, recursive: bool) -> Vec<PathBuf> {
    let mut repos = Vec::new();

    let walker = WalkDir::new(root)
        .max_depth(if recursive { usize::MAX } else { 2 })
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in walker {
        if entry.file_name() == ".git" && entry.file_type().is_dir() {
            if let Some(parent) = entry.path().parent() {
                repos.push(parent.to_path_buf());
            }
        }
    }
    repos
}

fn do_maintenance_on(repo: &str) -> () {
    let cwd = std::env::current_dir().unwrap();
    set_current_dir(repo).expect(format!("No such directory: {repo}").as_str());
    run_gc_on(repo);
    prune_branches(repo);
    print_stash_count_for(repo);
    set_current_dir(cwd.as_path()).unwrap();
}

fn run_gc_on(repo: &str) -> () {
    Command::new("git")
        .arg("gc")
        .arg("--aggressive")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(format!("Failed to clean {repo}").as_str());
}

fn prune_branches(repo: &str) -> () {
    Command::new("git")
        .arg("fetch")
        .arg("--prune")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(format!("Failed to prune branches from {repo}").as_str());
}

fn print_stash_count_for(repo: &str) -> () {
    let count = get_stash_count_for(repo);
    if count > 0 {
        println!("{}", get_stash_warning_message(repo, count));
    }
}

fn get_stash_count_for(repo: &str) -> i32 {
    let stash_cmd = Command::new("git")
        .arg("stash")
        .arg("list")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect(format!("Failed to run git stash list on {repo}").as_str());
    let count = Command::new("wc")
        .arg("-l")
        .stdin(Stdio::from(stash_cmd.stdout.unwrap()))
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .output()
        .expect(format!("Failed to count stash entries on {repo}").as_str());
    String::from_utf8(count.stdout)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap()
}

fn get_stash_warning_message(repo: &str, count: i32) -> String {
    let yellow_repo = repo.yellow();
    let yellow_count = count.to_string().yellow();
    let stash_entry_str = if count == 1 {
        "stash entry"
    } else {
        "stash entries"
    };
    format!(
        "{} {} {} {}",
        yellow_repo,
        "has".yellow(),
        yellow_count,
        stash_entry_str.yellow()
    )
}
