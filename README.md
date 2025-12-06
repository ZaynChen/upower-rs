# upower-rs
The Rust bindings of upower

## Recommandation

using [zbus][zbus] instead of this crate, because [zbus][zbus] provides
Rust API for D-Bus communication, and this crate is a glib C bindings.

## Usage

```toml
upower = { git = "https://github.com/ZaynChen/upower-rs.git", version = "0.1.0" }
```

[zbus]: https://github.com/z-galaxy/zbus "zbus"
