watch *args:
	cargo-watch -c -x "run -- {{args}}" 2> /dev/null

run *args:
	cargo run -- {{args}}

build *args:
	cargo-watch -c -x "build"

clean:
	cargo clean -p spotify-client-tui

logs:
	./scripts/log.sh 2> /dev/null
