use clap::{App, Arg};

pub fn build<'a, 'b>() -> App<'a> {
    App::new("")
        // package metadata from Cargo
        .name(env!("CARGO_PKG_NAME"))
        .about("htmanager is a high end website project manager written in Rust and is 100% open source forever. Feel free to contribute! 

License GPLv3+: GNU GPL version 3 or later
htmanager  Copyright (C) 2022  Matteo Rosato
This program comes with ABSOLUTELY NO WARRANTY; This is free software, and you are welcome to 
redistribute it under certain conditions

")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            App::new("serve")
                .about("Serve project for local development on 127.0.0.1 and localhost 
		[WARNING: The serve function is in beta!]")
		.arg(
                    Arg::with_name("port")
                        .help("the port to run the dev server on")
                        .short('p')
                        .long("port")
                        .takes_value(true)
                        .default_value("8080")
                )
		.arg(
    		    Arg::with_name("host")
        	        .long("host")
        		.takes_value(false)
        		.help("Hosts on LAN at 192.168.1.186:[PORT]")
		))
		.subcommand(
		App::new("new")
                .about("Create new project")
		.arg(
			Arg::with_name("directory_name")
                        .help("The name of your project directory")
                        .short('n')
                        .long("name")
                        .takes_value(true)
			.default_value("")
		))
		.subcommand(
                App::new("build-scss")
                .about("builds scss/sass")
                .arg(
                     	Arg::with_name("source")
                        .help("The name of the file you want to compile")
                        .short('s')
                        .long("source")
                        .takes_value(true)
                        .default_value("")

		)
		.arg(
			Arg::with_name("output")
			.help("name of the output css file")
			.short('o')
			.long("output")
			.takes_value(true)
			.default_value("output.css")
		))
}
