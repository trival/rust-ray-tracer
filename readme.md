# ray tracer in rust

to compile and run

```bash
cargo run --release --example scene1 > out/scene1.ppm
cargo run --release --example scene2 > out/scene2.ppm
# or with timing
time cargo run --release --example scene1 > out/scene1.ppm
time cargo run --release --example scene2 > out/scene2.ppm
```

rerun on change with [watchexec](https://github.com/watchexec/watchexec):

```bash
watchexec -e rs 'time cargo run --release --example scene1 > out/scene1.ppm'
watchexec -e rs 'time cargo run --release --example scene2 > out/scene2.ppm'
```
