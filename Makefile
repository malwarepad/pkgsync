all:
	cargo build --release

clean:
	cargo clean

install:
	cp target/release/pkgsync /usr/bin
