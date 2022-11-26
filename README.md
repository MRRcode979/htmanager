# htmanager

htmanager is a high end website project manager written in Rust and is 100% open source forever.

**Please feel free to contribute!**

## What can htmanager do?

htmanager can do the following things:
Serve html files!
Compile scss to css!
Create html projects!

#### Note:
if you use external sources from your directory (Like a script tag linking an external javascript file)
it won't work because htmanager does not serve the whole directory this should be fixed soon though.

## TODO:
Add directory listing to the serve function.
Add multithreading to the serve function.
Makefile / installation script?

# Install 
first clone this repository

next go into the project directory

then run: cargo build --relase

finally run the executable and test if it's working by typing:

./target/release/htmanager --help


That is it for now! Feel free to contribute.
