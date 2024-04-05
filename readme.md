# ray tracer in rust

to compile and run

```bash
cargo run --release --example scene1 > out/scene1.ppm
cargo run --release --example scene2 > out/scene2.ppm
# or with timing
time cargo run --release --example scene1 > out/scene1.ppm
time cargo run --release --example scene2 > out/scene2.ppm
```

or use the bun script

```bash
bun render.ts scene1
bun render.ts -n 10 scene2 # renders 10 versions
bun render.ts -n 3 --timestamp scene3 # renders 3 versions with timestamp in the name
bun render.ts --help
```

rerun on change with [watchexec](https://github.com/watchexec/watchexec):

```bash
watchexec -e rs 'time cargo run --release --example scene1 > out/scene1.ppm'
watchexec -e rs 'time cargo run --release --example scene2 > out/scene2.ppm'
```
