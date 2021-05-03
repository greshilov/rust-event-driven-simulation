## rust-event-driven-simulation

A wasm-library to simulate 2D-particle collision systems.

## Build instructions
* Install [wasm pack](https://rustwasm.github.io/wasm-pack/installer/).
* Build project using following command.
```
wasm-pack build
```

## Testing
```
cargo test
```

## Benchmarking
```
cargo +nightly bench
```

One can use `perf` to analyze perfomance.
```
RUSTFLAGS=-g cargo +nightly bench simulation_ticks_crowded

# Example output:
# Running unittests (target/release/deps/bench-8460f07c689bfb64)

perf record -g target/release/deps/bench-8460f07c689bfb64 --bench
perf report
```

Results on my i7-4702M are:
```
test simulation_ticks         ... bench:       5,375 ns/iter (+/- 282)
test simulation_ticks_crowded ... bench:   1,186,021 ns/iter (+/- 84,353)
```