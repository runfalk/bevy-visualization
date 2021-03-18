Bevy Visualizer
===============
This is an experimental project to determine the viability of using Rust and
Bevy for visualizing the output of discrete event simulations. Bevy offers some
very interesting features like web, Android and iOS support. It also offers
things like lighting, 3D model support, easy texturing, modern shaders and
transparency. It's ECS approach is also multi-threaded by default, which is
really interesting.

This feature set could make Bevy an interesting platform not only for
visualization, but also for machine learning training.


Development environment
-----------------------
The code is tested on Rust 1.50.

See [Bevy requirements](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
for which dependencies that are needed.

```bash
# Fedora
sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel
```

Run the simulation using cargo. It might not be quick enough unless it runs in
release mode.

```bash
cargo run --release
```
