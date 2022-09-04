use std::fs;
use std::io::Write;

pub fn build_scss(p: &str, o: &str) -> Result<(), Box<grass::Error>>
{
    let scss = grass::from_path(p, &grass::Options::default())?;
    let mut file = fs::File::create(o)?;

    file.write_all(scss.as_bytes())?;
    println!("Done.");
    Ok(())
}
