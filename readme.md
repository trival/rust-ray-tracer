# ray tracer in rust

to compile and run

```bash
cargo run --release --bin scene1 > scene1/out/output.ppm
cargo run --release --bin scene2 > scene2/out/output.ppm
# or with timing
time cargo run --release --bin scene1 > scene1/out/output.ppm
time cargo run --release --bin scene2 > scene2/out/output.ppm
```

rerun on change with [watchexec](https://github.com/watchexec/watchexec):

```bash
watchexec -e rs 'cargo run --release --bin scene1 > scene1/out/output.ppm'
watchexec -e rs 'cargo run --release --bin scene2 > scene2/out/output.ppm'
```
