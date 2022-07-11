#![allow(non_snake_case)]

use colorful::Colorful;
use git_config::{fs::Config, File};
use std::process::Command;

use crate::utils::exec;

fn getCurrentBranch() -> Result<String, String> {
    return exec(Command::new("git").arg("branch").arg("--show-current"));
}

pub fn stageFiles(paths: &Vec<String>) -> () {
    let result = exec(Command::new("git").arg("add").args(paths));

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error.bg_red()),
    };
}

pub fn unstageFiles(paths: &Vec<String>) -> () {
    let result = exec(Command::new("git").arg("reset").args(paths));

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error.bg_red()),
    };
}

pub fn commit(message: &Vec<String>, ammend: &bool) -> () {
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

pub fn link(remote: &String, name: &Option<String>) -> () {
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

pub fn push(remoteName: &Option<String>) -> () {
    let mut cmd = Command::new("git");

    cmd.arg("push")
        .arg("-u")
        .arg(remoteName.as_ref().unwrap_or(&String::from("origin")))
        .arg(getCurrentBranch().unwrap());

    let result = exec(&mut cmd);

    match result {
        Ok(output) => println!("{}", output),
        Err(error) => eprintln!("{}", error.bg_red()),
    };
}
