#![allow(non_snake_case)]

use std::process::Command;

use clap::{Parser, Subcommand};

use colorful::Colorful;

#[derive(Parser, Debug)]
#[clap(author, version, propagate_version = true, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Initialize a new git repository")]
    Init {},
    #[clap(about = "Stages the given files' changes (alias: add)", alias = "add")]
    Stage {
        #[clap(value_parser)]
        paths: Vec<String>,
    },
    #[clap(about = "Unstages the given files' changes (aliases: remove, rm)", aliases = &["remove", "rm"])]
    Unstage {
        #[clap(value_parser)]
        paths: Vec<String>,
    },
    #[clap(about = "Commits the staged changes (alias: save)", alias = "save")]
    Commit {
        #[clap(value_parser)]
        message: Vec<String>,

        #[clap(short, long)]
        ammend: bool,
    },
    #[clap(about = "Links this repository to a remote repository")]
    Link {
        #[clap(value_parser)]
        remoteUrl: String,

        #[clap(value_parser)]
        name: Option<String>,
    },
}

fn main() {
    let args: Args = Args::parse();

    match &args.command {
        Commands::Init {} => println!("git init"),
        Commands::Stage { paths } => stageFiles(paths),
        Commands::Unstage { paths } => unstageFiles(paths),
        Commands::Commit { message, ammend } => commit(message, ammend),
        Commands::Link { remoteUrl, name } => link(remoteUrl, name),
    }
}

fn stageFiles(paths: &Vec<String>) -> () {
    let result = exec(Command::new("git").arg("add").args(paths));

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error.bg_red()),
    };
}

fn unstageFiles(paths: &Vec<String>) -> () {
    let result = exec(Command::new("git").arg("reset").args(paths));

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error.bg_red()),
    };
}

fn commit(message: &Vec<String>, ammend: &bool) -> () {
    let mut cmd = Command::new("git");

    cmd.arg("commit")
        .arg("-m")
        .arg(format!("\"{}\"", message.join(" ")));

    if *ammend {
        cmd.arg("--amend");
    }

    let result = exec(&mut cmd);

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error.bg_red()),
    };
}

fn link(remote: &String, name: &Option<String>) -> () {
    let mut cmd = Command::new("git");

    cmd.arg("remote")
        .arg("add")
        .arg(name.as_ref().unwrap_or(&String::from("origin")))
        .arg(remote);

    let result = exec(&mut cmd);

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error.bg_red()),
    };
}

fn exec(command: &mut Command) -> Result<String, String> {
    let result = command.output().unwrap_or_else(|err| {
        panic!("{}", err.to_string());
    });

    if result.status.success() {
        return Ok(match result.stdout.is_empty() {
            true => String::from(""),
            false => String::from_utf8_lossy(&result.stdout.split_last().unwrap().1).to_string(),
        });
    } else {
        return Err(String::from_utf8_lossy(&result.stderr.split_last().unwrap().1).to_string());
    }
}
