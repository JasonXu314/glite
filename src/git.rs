#![allow(non_snake_case)]

use colorful::Colorful;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::read_to_string;
use std::process::Command;

use crate::utils::exec;

pub struct Remote {
    pub name: String,
    pub url: String,
}

pub struct Branch {
    pub name: String,
    pub remote: Option<String>,
    pub current: bool,
}

pub struct Config {
    remotes: Vec<Remote>,
    branches: Vec<Branch>,
}

impl Config {
    pub fn parse(configStr: &String) -> Config {
        lazy_static! {
            static ref REMOTE_REGEX: Regex = Regex::new("^\\[remote \"(.+)\\]$").unwrap();
            static ref BRANCH_REGEX: Regex = Regex::new("^\\[branch \"(.+)\\]$").unwrap();
            static ref KEY_VALUE_REGEX: Regex = Regex::new("^(.+)\\s*=\\s*(.+)$").unwrap();
        }

        let mut config = Config {
            remotes: Vec::new(),
            branches: Vec::new(),
        };

        let lines = configStr.lines();
        let mut currRemote: Option<Remote> = None;
        let mut currBranch: Option<Branch> = None;
        for mut line in lines {
            line = line.trim();

            if REMOTE_REGEX.is_match(line) {
                if currBranch.is_some() {
                    config.branches.push(currBranch.unwrap());
                    currBranch = None;
                } else if currRemote.is_some() {
                    config.remotes.push(currRemote.unwrap());
                }

                let groups = REMOTE_REGEX.captures(line).unwrap();
                let name = groups[1].to_string();

                currRemote = Some(Remote {
                    name,
                    url: String::new(),
                });
            } else if BRANCH_REGEX.is_match(line) {
                if currBranch.is_some() {
                    config.branches.push(currBranch.unwrap());
                } else if currRemote.is_some() {
                    config.remotes.push(currRemote.unwrap());
                    currRemote = None;
                }

                let groups = BRANCH_REGEX.captures(line).unwrap();
                let name = groups[1].to_string();
                let branchName = name.clone();

                currBranch = Some(Branch {
                    name,
                    remote: None,
                    current: branchName == getCurrentBranch().unwrap(),
                });
            } else if KEY_VALUE_REGEX.is_match(line) {
                let groups = KEY_VALUE_REGEX.captures(line).unwrap();
                let key = groups[1].to_string();
                let value = groups[2].to_string();

                if key == "url" {
                    currRemote.as_mut().unwrap().url = value;
                } else if key == "remote" {
                    currBranch.as_mut().unwrap().remote = Some(value);
                }
            }
        }

        if currBranch.is_some() {
            config.branches.push(currBranch.unwrap());
        } else if currRemote.is_some() {
            config.remotes.push(currRemote.unwrap());
        }

        return config;
    }
}

pub struct Git {
    config: Config,
}

impl Git {
    pub fn new() -> Result<Git, String> {
        let configStr = match read_to_string(".git/config") {
            Ok(s) => s,
            Err(_) => return Err(String::from("Could not read .git/config")),
        };

        let config = Config::parse(&configStr);
        return Ok(Git { config });
    }

    pub fn stageFiles(&self, paths: &Vec<String>) -> () {
        let result = exec(Command::new("git").arg("add").args(paths));

        match result {
            Ok(output) => println!("{}", output),
            Err(error) => eprintln!("{}", error.bg_red()),
        };
    }
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

fn getCurrentBranch() -> Result<String, String> {
    return exec(Command::new("git").arg("branch").arg("--show-current"));
}
