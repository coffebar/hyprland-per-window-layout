# Hyprland per window xkb layout

The script maintains associative array `windows` that maps windows' addresses to selected layouts.

It's fork of [hyprland-per-window-layout](https://github.com/MahouShoujoMivutilde/hyprland-per-window-layout), with zero configuration and multi-keyboard support.

## How to use it

Install **hyprland-per-window-layout** from [AUR](https://aur.archlinux.org/packages/hyprland-per-window-layout)

and

Add this line to your hyprland.conf

```
exec-once = /usr/bin/hyprland-per-window-xkblayout
```

## Requirements

* [Hyprland](https://github.com/hyprwm/Hyprland)
* bash 4.0+ (for associative arrays).
* socat (for listening for Hyprland socket2 events).
* [gojq](https://github.com/itchyny/gojq) (for working with `hyprctl`'s json. `jq` could work, but it is much slower).

-----

Tested and works on Hyprland v0.21.0beta.
