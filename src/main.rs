extern crate clap;
extern crate colored;

mod config;
mod model;

use std::fs;
use std::path::Path;
use model::container_compose::ContainerCompose;
use config::{Config, ActionType};

fn main() {
    let default_filenames: Vec<&str> = vec![
    "docker-compose.yml",
    "docker-compose.yaml",
    "container-compose.yml",
    "container-compose.yaml"
    ];
    let config = Config::new();
    let mut filename = config.filename;

    if filename == "default"{
        filename = default_filenames.into_iter()
            .filter(|f| Path::new(f).exists() == true)
            .nth(0)
            .expect("Error: composer file not found.\nYou can use --help if you need information about the usage of this program")
            .to_string();
        
    }

    let yaml_str = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");

    let mut container_compose = ContainerCompose::new(yaml_str);

    match config.action{
        ActionType::Up =>{
            container_compose.up(config.up_arguments);
        },
        _ =>{}
    }
}
