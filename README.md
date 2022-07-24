# Ray-Tracing-in-One-Weekend (w/ Rust)

A Rust implementation of [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## Usage

1. Build the project.

```sh
cargo build --release
```

2. Run (and time) the release binary.

```sh
time ./target/release/tracer
```

3. Check `static/complex_scene.ppm` for the generated image.
