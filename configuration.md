# Configuration

## Reason to have configuration

This program can be used without a configuration file. But you may want to have a configuration file to:

- Set up a keyboard layout for a specific window classes

## Configuration file

Create a file
~/.config/hyprland-per-window-layout/options.toml

Example configuration file:

```toml
# list of keyboards to operate on
# use `hyprctl devices -j` to list all keyboards
keyboards = [
  "lenovo-keyboard",
]

# layout_index => window classes list
# use `hyprctl clients` to get class names
[[default_layouts]]
1 = [
    "org.telegram.desktop",
]
```

This example will set your second layout for the Telegram by default.

1 - is a layout index. In case of this input configuration:
```
input {
  kb_layout = us,es,de
  ...
```
*us* index is 0, *es* index is 1, *de* index is 2.

Note, *keyboards* section is required for default_layouts feature.

Here is more complex example if you have 3 layouts and 2 keyboards:

```toml
# list of keyboards to operate on
# use `hyprctl devices -j` to list all keyboards
keyboards = [
  "apple-magic-keyboard",
  "lenovo-keyboard",
]

# layout_index => window classes list
# use `hyprctl clients` to get class names
[[default_layouts]]
1 = [
    "org.telegram.desktop",
    "discord",
]
2 = [
    "firefox",
]
```
