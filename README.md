# htlnm

A cli tool for [notenmanagement.htl-braunau.at](https://notenmanagement.htl-braunau.at/) written in Rust (based on a class assignment).

## Prereqs

- [rustc and cargo](https://www.rust-lang.org/tools/install)

## Build and install

```bash
cargo install --path .
```

## Example usage

```bash
# Print an overview of all commands and flags
htlnm -h

# Print help menu for data that can be fetched
htlnm get -h

# Login to the API
htlnm -l -u john.doe -p 'myPA$$W0RD'

# Print the login response (can returned cached data without the -l flag)
htlnm get login

# Fetch student data
htlnm get student
```

Then login response is cached in the app's config file: `~/.config/htlnm/config.json` ([path depending on the OS](https://docs.rs/dirs/latest/dirs/fn.config_local_dir.html)).

## Development

```bash
# install cargo watch
cargo install cargo-watch

# rerun tool when a file changes
cargo watch -x 'run -- get student'
```
