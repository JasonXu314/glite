#![allow(non_snake_case)]

mod git;
mod utils;

use clap::{Parser, Subcommand};

use git::Git;

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
    #[clap(
        about = "Pushes the staged changes to the remote repository on the corresponding branch"
    )]
    Push {
        #[clap(value_parser)]
        remoteName: Option<String>,
    },
    #[clap(about = "Manipulates branches")]
    Branch {
        #[clap(value_parser)]
        name: String,

        #[clap(short, long)]
        checkout: bool,
    },
    #[clap(about = "Checks out the given branch")]
    Checkout {
        #[clap(value_parser)]
        name: String,
    },
    #[clap(about = "Pulls changes from the remote branch to local")]
    Pull {},
}

fn main() {
    let args: Args = Args::parse();
    let git = Git::new().unwrap();

    match &args.command {
        Commands::Init {} => git.init(),
        Commands::Stage { paths } => git.stageFiles(paths),
        Commands::Unstage { paths } => git.unstageFiles(paths),
        Commands::Commit { message, ammend } => git.commit(message, ammend),
        Commands::Link { remoteUrl, name } => git.link(remoteUrl, name),
        Commands::Push { remoteName } => git.push(remoteName),
        Commands::Branch { name, checkout } => git.branch(name, checkout),
        Commands::Checkout { name } => git.checkout(name),
        Commands::Pull {} => git.pull(),
    }
}
