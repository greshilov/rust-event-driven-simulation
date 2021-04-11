## red-server

## Build instructions
This crate uses rocket, so you have to switch to a night channel.
```bash
rustup override set nightly
```
Create `.env` file from `example.env`.  
```bash
cp example.env .env
```
`SECRET_KEY` must be the same as used for the `red-simulation` crate.

## Testing
```
cargo test
```
