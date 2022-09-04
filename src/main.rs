mod new;
mod server;
mod cli;
mod build_scss;

use build_scss::*;
use cli::*;
use server::*;
use new::*;


use colored::Colorize;
use clap::{App, Arg};

fn main() {
let matches = cli::build().get_matches();
if let Some(ref matches) = matches.subcommand_matches("build-scss") {
if matches.value_of("source") == Some("") {
println!("{} {}", "Error:".red().bold(), "No scss source file specified use the -s flag then the filename".cyan());
std::process::exit(0);
} else {
build_scss(matches.value_of("source").unwrap(), matches.value_of("output").unwrap());
}
}
if let Some(ref matches) = matches.subcommand_matches("new") {
if matches.value_of("directory_name") == Some("") {
println!("{} {}", "Error:".red().bold(), "No project name specified use the --name flag to add a name.".cyan());
std::process::exit(0);
} else {
create_project(matches.value_of("directory_name").unwrap());
}
}

if let Some(ref matches) = matches.subcommand_matches("serve") {
if matches.value_of("source") == Some("") {
println!("{} {}", "Error:".red().bold(), "No html source file specified use the -s flag then the filename".cyan());
std::process::exit(0);
} else {
serve(matches.value_of("source").unwrap(), matches.value_of("port").unwrap());
}
}
}
