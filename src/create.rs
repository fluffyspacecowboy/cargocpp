#![allow(unused_parens)]

use std::fs;
use std::process;
use colored::Colorize;

pub fn directory(path: &String) -> std::io::Result<()> {
	
	if(fs::exists(path)?) {
		println!("{}, project already exists", "Failed to create new project".red().bold());
		process::exit(101);
	}
	
	fs::create_dir_all(format!("{path}/src"))?;
	fs::create_dir_all(format!("{path}/include"))?;
	fs::create_dir_all(format!("{path}/lib"))?;
	
	println!("{}, created {path}", "Success!".green().bold());
	Ok(())
}
