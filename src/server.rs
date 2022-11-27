mod readfile;

use readfile::*;
use colored::Colorize;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn serve(filename: &str, port: &str) {
    println!("{} {}", "WARNING:".yellow().bold(), "The serve function of htmanager is still in heavy development".bold());
    println!("local (single threaded) dev server hosted at port {} (^C to force shutdown dev server)", port.cyan().italic());

    let listener = TcpListener::bind("localhost:".to_owned() + port).unwrap(); 

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, filename);
    }
}

fn handle_connection(mut stream: TcpStream, path: &str) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
        if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
  
        let file = Filename::from_path(path);
        let contents = fs::read_to_string(path).unwrap();
        let length = contents.len();
		
	let response = 
	   format!(
	   "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
	   );

        stream.write_all(response.as_bytes()).unwrap();
}
}
