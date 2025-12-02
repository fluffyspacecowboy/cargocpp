use clap::{Arg, Command};
use crate::create;
use crate::download;

fn args() -> Command {
    Command::new("cpp")
        .about("cargo but for c++")
        .arg_required_else_help(true)
        .arg(
            Arg::new("c")
                .long("create")
                .help("Create new project")
                .num_args(1)
                .value_name("PROJECT NAME"),
        )
				.arg(
					Arg::new("a") 
					.long("add")
					.help("Install new libraries")
					.num_args(1)
					.value_name("LIBRARY NAME")
				)
}

pub fn check_for_arg() {
    let args = args().get_matches();

    if let Some(name) = args.get_one::<String>("c") {
        println!("Creating project: {name}");

				let result = create::directory(name);

				match result {
					Ok(()) => {
						println!("{name} created succesfully!");
					}
					Err(e) => {
						eprintln!("Creating {name} failed, {e}")
					}
				}
    }

		if let Some(name) = args.get_one::<String>("a") {
			println!("implement downloading package from github");
			let include_result = download::include(name);
			let library_result = download::library(name);

			match include_result {
				Ok(()) => {
					println!("{name} includes were installed succesfully!");
				}	
				Err(e) => {
					println!("{name} includes installation failed, {e}");
				}
			}
			match library_result {
				Ok(()) => {
					println!("{name} libraries were installed succesfully!");
				}	
				Err(e) => {
					println!("{name} libraries installation failed, {e}");
				}
			}
		}
}

