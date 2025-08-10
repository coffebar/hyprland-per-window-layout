// read and represent the options file
// located at ~/.config/hyprland-per-window-layout/options.toml

use std::fs::File;

use serde::Deserialize;
use std::collections::HashMap;
use toml::Table;

#[derive(Deserialize, Debug)]
pub struct Options {
    pub keyboards: Vec<String>, // list of keyboards to switch layouts on
    pub default_layouts: HashMap<u16, Vec<String>>, // default layouts for window classes
}

// function to read the options file toml
pub fn read_options() -> Options {
    // get the path to the options file
    // in $HOME/.config/hyprland-per-window-layout/options.toml
    let options_path = dirs::config_dir()
        .unwrap()
        .join("hyprland-per-window-layout")
        .join("options.toml");
    // read the file contents if it exists
    // ignore if it doesn't exist
    match File::open(&options_path) {
        Ok(_file) => {
            // read the file contents
            let file_content = match std::fs::read_to_string(&options_path) {
                Ok(content) => content,
                Err(e) => {
                    println!("Error reading options.toml: {e}");
                    return Options {
                        keyboards: Vec::new(),
                        default_layouts: HashMap::new(),
                    };
                }
            };
            let _t = match file_content.parse::<Table>() {
                Ok(table) => table,
                Err(e) => {
                    println!("Error parsing options.toml: {e}");
                    return Options {
                        keyboards: Vec::new(),
                        default_layouts: HashMap::new(),
                    };
                }
            };
            let mut map = HashMap::new();
            let mut keyboards = Vec::new();
            if let Some(_default_layouts) = _t.get("default_layouts") {
                if let Some(default_layouts_array) = _default_layouts.as_array() {
                    if let Some(first_layout) = default_layouts_array.first() {
                        if let Some(layout_table) = first_layout.as_table() {
                            for (key, value) in layout_table.iter() {
                                if let Ok(key_num) = key.parse::<u16>() {
                                    if let Some(value_array) = value.as_array() {
                                        let layout_vec: Vec<String> = value_array
                                            .iter()
                                            .filter_map(|x| x.as_str().map(|s| s.to_string()))
                                            .collect();
                                        map.insert(key_num, layout_vec);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if let Some(_keyboards) = _t.get("keyboards") {
                if let Some(keyboards_array) = _keyboards.as_array() {
                    for keyboard in keyboards_array.iter() {
                        if let Some(keyboard_str) = keyboard.as_str() {
                            keyboards.push(keyboard_str.to_string());
                        }
                    }
                }
            }
            return Options {
                keyboards,
                default_layouts: map,
            };
        }
        Err(_) => {
            println!("options.toml not found, using defaults");
        }
    };
    Options {
        keyboards: Vec::new(),
        default_layouts: HashMap::new(),
    }
}
