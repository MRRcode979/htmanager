use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

const DEFAULT_HTML: &str = "
<!DOCTYPE html>
<html>
    <head>
	<link rel='stylesheet' href='styles.css'>
        <title>My Page</title>
    </head>
    <body>
        <h1>Heading</h1>
        <article id='article1'>
            <h2 id='article-head' class='header'>Hello World</h2>
            <p>This is a simple HTML demo made by htmanager. Edit this html to make a website!</p>
        </article>
      <script src='./main.js' type='text/javascript'></script>
    </body>
</html>";

pub fn create_project(dirname: &str) -> std::io::Result<()> {
    let mut files = vec!["/index.html", "/index.js", "/styles.css"];

    let mut scss = String::new();

    println!("Include scss? [Y/n]");

    io::stdin()
        .read_line(&mut scss)
        .expect("Failed to read line");

    if scss.trim() == "Y".trim() || scss.trim() == "y".to_string() {
        files.push("/styles_scss.scss");
        println!("Done.");
    } else {
        println!("Done.");
    }

    fs::create_dir_all(dirname);

    let mut i: u8 = 0;

    for mut i in 0..files.len() {
        fs::File::create(dirname.to_owned() + files[i]);
        i += 1;
    }

    let mut html_default = OpenOptions::new()
        .append(true)
        .open(dirname.to_owned() + "/index.html")
        .expect("Unable to open file");

    html_default
        .write_all(DEFAULT_HTML.as_bytes())
        .expect("Failed to write html layout");

    Ok(())
}
