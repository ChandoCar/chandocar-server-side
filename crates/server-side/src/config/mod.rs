use serde::{Deserialize, Serialize};
use std::io::Write;
use std::sync::LazyLock;
use std::{fs, path};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {

}

// TODO: Add support for more platform if needed
const CONF_FILE_PATH: &str = {
    #[cfg(debug_assertions)]
    {
        "config.json"
    }

    #[cfg(not(debug_assertions))]
    {
        "/etc/chandocar/config.json"
    }
};

static CONFIG: LazyLock<Config> = LazyLock::new(|| match load_config_from_file() {
    Ok(config) => config,
    Err(e) => panic!("{}", e),
});

pub fn config() -> &'static Config {
    &CONFIG
}

fn load_config_from_file() -> Result<Config, String> {
    let conf_file_path = path::Path::new(CONF_FILE_PATH);

    if !conf_file_path.exists() {
        fs::File::create_new(conf_file_path)
            .map_err(|e| format!("Error creating the config.json file: {}", e))?
            .write_all(
                serde_json::to_string_pretty(&Config::default())
                    .map_err(|e| format!("Error converting the default config to JSON: {}", e))?
                    .as_bytes(),
            )
            .map_err(|e| {
                format!(
                    "Error writting the default config to the config.json file: {}",
                    e
                )
            })?
    }

    serde_json::from_reader(
        fs::File::open(conf_file_path)
            .map_err(|e| format!("config.json file opening error: {}", e))?,
    )
    .map_err(|e| format!("Error parsing config.json file content: {}", e))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config() {
        let conf_file_path = path::Path::new(CONF_FILE_PATH);

        if conf_file_path.exists() {
            fs::remove_file(conf_file_path).expect("Config file should be removed")
        }

        // Creating the config file
        config();

        // Getting the config once the file is created
        config();
    }
}
