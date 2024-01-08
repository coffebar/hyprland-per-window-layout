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
            let _t = std::fs::read_to_string(&options_path)
                .unwrap()
                .parse::<Table>()
                .unwrap();
            let mut map = HashMap::new();
            let mut keyboards = Vec::new();
            if let Some(_default_layouts) = _t.get("default_layouts") {
                for (key, value) in _default_layouts[0].as_table().unwrap().iter() {
                    map.insert(
                        key.parse::<u16>().unwrap(),
                        value
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|x| x.as_str().unwrap().to_string())
                            .collect::<Vec<String>>(),
                    );
                }
            }
            if let Some(_keyboards) = _t.get("keyboards") {
                for keyboard in _keyboards.as_array().unwrap().iter() {
                    keyboards.push(keyboard.as_str().unwrap().to_string());
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
    return Options {
        keyboards: Vec::new(),
        default_layouts: HashMap::new(),
    };
}
