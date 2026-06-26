# swayopacity

A lightweight sway daemon that adjusts the opacity based on focus. Freely adjust the opaqueness of focused and unfocused windows. If you want additional features or find a bug, feel free to open an issue or a pull request.

## Requirements

- Sway
- Rust & Cargo

## Installation

```bash
cargo install swayopacity
```

## Configuration

On first launch, swayopacity creates a configuration file at:

`~/.config/swayopacity/config.toml`

The default values are:

```toml
default_opacity = 0.85  # opacity of unfocused windows
target_opacity = 1.0    # opacity of focused windows
```

## Autostart

**With systemd** (recommended):

```bash
mkdir -p ~/.config/systemd/user
cp systemd/swayopacity.service ~/.config/systemd/user/
systemctl --user enable swayopacity
systemctl --user start swayopacity
```

**With Sway config** (simple):

Add to `~/.config/sway/config`:

```sway
exec swayopacity
```

## License

MIT
