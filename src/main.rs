use clap::Parser;
use colored::Colorize;
use std::env::set_current_dir;
use std::process::{Command, Stdio};

/// A simple program for performing maintenance on Git repositories
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Relative paths to the Git repositories to maintain
    #[arg(short, long, value_delimiter = ',')]
    repos: Vec<String>,
}

fn main() {
    let args = Args::parse();
    for repo in args.repos {
        do_maintenance_on(&repo);
    }
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
    String::from_utf8(count.stdout).unwrap().trim().parse::<i32>().unwrap()
}

fn get_stash_warning_message(repo: &str, count: i32) -> String {
    let yellow_repo = repo.yellow();
    let yellow_count = count.to_string().yellow();
    let stash_entry_str = if count == 1 { "stash entry" } else { "stash entries" };
    format!("{} {} {} {}", yellow_repo, "has".yellow(), yellow_count, stash_entry_str.yellow())
}
