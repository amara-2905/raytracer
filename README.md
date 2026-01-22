# Raytracer 
![final render](image.png)
<br>
This image was created at 1200x675 at 500 samples per pixel. It took 17 minutes 45 seconds to run on my machine on a release build.

```bash
cargo build --release
./target/release/raytracer > trial.ppm
```
Builds the project in release mode and runs the renderer, redirecting the output to `trial.ppm`.