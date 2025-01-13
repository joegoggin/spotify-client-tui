# Spotify Client TUI 

## Scripts

Note that in order for the following commands to work you will need to have both
[just](https://github.com/casey/just) and 
[cargo-watch](https://github.com/watchexec/cargo-watch) installed.

### run

Run is a shortcut for the following command `cargo run --`. 

```
just run control pause-play
```

### build

Build will build your project in watch mode using 
[cargo-watch](https://github.com/watchexec/cargo-watch).
This mean your code will rebuild anytime you save your code.

```
just build
```

### watch 

Watch will run your project in watch mode using 
[cargo-watch](https://github.com/watchexec/cargo-watch).
Note that this command will not output and stderr. This is to avoid conflicts
with the output of the TUI app. If you need to see the stderr output you use
`just build` to see the stderr output. 

```
just watch 
```

### clean

Clean is a shortcut for the following command `cargo clean -p spotify-client-tui`

```
just clean
```

### logs

Logs will display live logs for the app.

```
just logs
```
