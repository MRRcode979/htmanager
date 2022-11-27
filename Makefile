install:
	echo "installing..."
	cargo build --release
	sudo mv ./target/release/htmanager /usr/bin
	echo "Done!"
	htmanager --help

uninstall:
	sudo rm -fv /usr/bin/htmanager
