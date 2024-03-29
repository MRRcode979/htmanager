mod build_scss;
mod cli;
mod new;
mod server;

use build_scss::*;
use cli::*;
use new::*;
use server::*;

use clap::{App, Arg};
use colored::Colorize;

fn main() {
    let mut adrr: [u8; 4];
    let matches = cli::build().get_matches();
    if let Some(ref matches) = matches.subcommand_matches("build-scss") {
        if matches.value_of("source") == Some("") {
            println!(
                "{} {}",
                "Error:".red().bold(),
                "No scss source file specified use the -s flag then the filename".cyan()
            );
            std::process::exit(0);
        } else {
            build_scss(
                matches.value_of("source").unwrap(),
                matches.value_of("output").unwrap(),
            );
        }
    }
    if let Some(ref matches) = matches.subcommand_matches("new") {
        if matches.value_of("directory_name") == Some("") {
            println!(
                "{} {}",
                "Error:".red().bold(),
                "No project name specified use the --name flag to add a name.".cyan()
            );
            std::process::exit(0);
        } else {
            create_project(matches.value_of("directory_name").unwrap());
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("serve") {
    if matches.is_present("host") {
	adrr = get_ip().expect("Error getting IP."); 
	println!("{:?}", adrr);
	serve(&adrr, matches.value_of("port").expect("Unable to parse argument.").parse().unwrap());
    } else {
	adrr = [127, 0, 0, 1];
	println!("{:?}", adrr);
	serve(&adrr, matches.value_of("port").expect("Unable to parse argument.").parse().unwrap());
    }
}
}
