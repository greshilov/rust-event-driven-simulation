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

## Database maintenance

### Create migration
```
diesel migration generate my_migration
```
Edit the migration files in the `migrations/` folder.

### Run migration
```
diesel migration run
```

### Revert migration
You always can revert created transaction using:
```
diesel migration revert
```

## Testing
```
cargo test
```
