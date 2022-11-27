use std::io::Write;
use std::io;
use std::fs;

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
            <p>This is a simple HTML demo made by htmanager edit this html to make a website!</p>
        </article>
      <script src='./main.js' type='text/javascript'></script>
    </body>
</html>";



pub fn create_project(dirname: &str) -> std::io::Result<()> {
    fs::create_dir_all(dirname);
    let mut html_file = fs::File::create(dirname.to_owned() + "/index.html")?;
    html_file.write_all(DEFAULT_HTML.as_bytes())?;
    
    fs::File::create(dirname.to_owned() + "/index.js");
    fs::File::create(dirname.to_owned() + "/styles.css");

    let mut scss = String::new();
    
    println!("Include scss? [Y/n]");
    
    io::stdin()
        .read_line(&mut scss)
        .expect("Failed to read line");
    
    if scss.trim() == "Y".trim() || scss.trim() == "y".to_string() {
    fs::File::create(dirname.to_owned() + "/styles_scss.scss");	
    } else {
    println!("Done.");
    }
    
    Ok(())
}
