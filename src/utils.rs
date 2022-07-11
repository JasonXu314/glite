#![allow(non_snake_case)]

use std::process::Command;

pub fn exec(command: &mut Command) -> Result<String, String> {
	let result = command.output().unwrap_or_else(|err| {
		panic!("{}", err.to_string());
	});

	if result.status.success() {
		return Ok(match result.stdout.is_empty() {
			true => String::from(""),
			false => {
				String::from_utf8_lossy(&result.stdout.split_last().unwrap().1).to_string()
			}
		});
	} else {
		return Err(
			String::from_utf8_lossy(&result.stderr.split_last().unwrap().1).to_string(),
		);
	}
}
