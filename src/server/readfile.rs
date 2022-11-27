use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::PathBuf;

pub struct Filename {
    path: PathBuf,
}

impl Filename {
   pub fn from_path(path: &str) -> Self {
        let mut fullpath = env::current_dir().unwrap();
        fullpath.push(&path[1..]);
        Self { path: fullpath }
    }
}
