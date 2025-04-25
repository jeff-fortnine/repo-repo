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
    if let Some(file) = args.file {
        for repo in read_to_string(file).unwrap().lines() {
            run_gc_on(repo);
        }
    } else if let Some(repos) = args.repos {
        for repo in repos {
            run_gc_on(&repo);
        }
    } else { println!("No file specified"); }
}

fn run_gc_on(repo: &str) -> () {
    let cwd = std::env::current_dir().unwrap();
    set_current_dir(repo).expect(format!("No such directory: {repo}").as_str());
    Command::new("git")
        .arg("gc")
        .arg("--aggressive")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect(format!("Failed to clean {repo}").as_str());
    set_current_dir(cwd.as_path()).unwrap();
}
