# Hyprland per window layout

![](https://img.shields.io/crates/d/hyprland-per-window-layout)
![](https://img.shields.io/github/issues-raw/coffebar/hyprland-per-window-layout)
![](https://img.shields.io/github/stars/coffebar/hyprland-per-window-layout)
![](https://img.shields.io/aur/version/hyprland-per-window-layout)
![](https://img.shields.io/crates/v/hyprland-per-window-layout)

#### No configuration is needed, just add your layouts (if you didn't yet) to the input section of hyprland config file and start this program right after Hyprland.

Written in Rust.

Note: it will not start if you don't have at least 2 keyboard layouts in hyprland.conf

## How to use

## Install **hyprland-per-window-layout** from [AUR](https://aur.archlinux.org/packages/hyprland-per-window-layout)

```bash 
# e.g.
yay -Sy && yay -S hyprland-per-window-layout
```

and

Add this line to your hyprland.conf

```
exec-once = /usr/bin/hyprland-per-window-layout
```

-----


## Install from cargo crates

```bash
cargo install hyprland-per-window-layout
```

Add this line to your hyprland.conf

```
exec-once = ~/.cargo/bin/hyprland-per-window-layout
```


## Install from source

Install from source with **rustup**:

```bash

git clone https://github.com/coffebar/hyprland-per-window-layout.git
cd hyprland-per-window-layout

rustup override set stable
rustup update stable

cargo build --release

mkdir -p ~/.local/bin/
cp target/release/hyprland-per-window-layout ~/.local/bin/

```
Add this line to your hyprland.conf

```
exec-once = ~/.local/bin/hyprland-per-window-layout
```

-----

## Contribution

Bug reports and PR are welcome. Thank you for your interest!

-----

Tested on Hyprland v0.32.
