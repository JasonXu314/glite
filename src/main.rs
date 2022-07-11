#![allow(non_snake_case)]

use std::process::Command;

use clap::{Parser, Subcommand};

use colorful::Colorful;

#[derive(Parser, Debug)]
#[clap(
    author = "Jason Xu",
    version,
    propagate_version = true,
    about = "A simple wrapper CLI for git"
)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Initialize a new git repository")]
    Init {},
    #[clap(about = "Stages the given files' changes", alias = "add")]
    Stage {
        #[clap(value_parser)]
        paths: Vec<String>,
    },
    #[clap(about = "Unstages the given files' changes", aliases = &["remove", "rm"])]
    Unstage {
        #[clap(value_parser)]
        paths: Vec<String>,
    },
    #[clap(about = "Commits the staged changes")]
    Commit {
        #[clap(value_parser)]
        message: Vec<String>,
    },
}

fn main() {
    let args: Args = Args::parse();

    match &args.command {
        Commands::Init {} => println!("git init"),
        Commands::Stage { paths } => stageFiles(paths),
        Commands::Unstage { paths } => unstageFiles(paths),
        Commands::Commit { message } => commit(message),
    }
}

fn stageFiles(paths: &Vec<String>) -> () {
    let result = Command::new("git").arg("add").args(paths).output().unwrap();

    if result.status.success() {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    } else {
        println!(
            "{}",
            String::from_utf8_lossy(&result.stderr.split_last().unwrap().1).bg_red()
        );
    }
}

fn unstageFiles(paths: &Vec<String>) -> () {
    let result = Command::new("git")
        .arg("reset")
        .args(paths)
        .output()
        .unwrap();

    if result.status.success() {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    } else {
        println!(
            "{}",
            String::from_utf8_lossy(&result.stderr.split_last().unwrap().1).bg_red()
        );
    }
}

fn commit(message: &Vec<String>) -> () {
    let result = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("\"{}\"", message.join(" ")))
        .output()
        .unwrap();

    if result.status.success() {
        println!("{}", String::from_utf8_lossy(&result.stdout));
    } else {
        println!(
            "{}",
            String::from_utf8_lossy(&result.stderr.split_last().unwrap().1).bg_red()
        );
    }
}
