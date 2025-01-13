watch *args:
	cargo-watch -c -x "run -- {{args}}"

run *args:
	cargo run -- {{args}}

build *args:
	cargo-watch -c -x "build"

clean:
	cargo clean -p spotify-client-tui
