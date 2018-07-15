# PingPong

PingPong is a small TCP/UDP latency testing tool similar to netperf written in Rust.

## Usage

Basic example of running a UDP test:

### Ponger (server):

```
$ cargo run --release --bin ponger -- --poll --pinger-addr 'localhost:20001' --ponger-addr 'localhost:20000' --udp
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/ponger --poll --pinger-addr 'localhost:20001' --ponger-addr 'localhost:20000' --udp`
```

Runs the ponger in polling mode (busy polling).

### Pinger (client):

```
$ cargo run --release --bin pinger -- -m 1000000 --warmup-messages 100000 --poll --pinger-addr '127.0.0.1:20001' --ponger-addr '127.0.0.1:20000' --udp
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/pinger -m 1000000 --warmup-messages 100000 --poll --pinger-addr '127.0.0.1:20001' --ponger-addr '127.0.0.1:20000' --udp`
Stats (ns): p50: 3458 p95: 4662 p99: 5911 p999: 15757
```

Runs the pinger in polling mode and sends a million messages after 100k warmup messages and prints latency percentiles after running.