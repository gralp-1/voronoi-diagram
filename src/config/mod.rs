use config::{Config, ConfigError, File};
use raylib::prelude::*;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub width:        i32,
    pub height:       i32,
    pub points:       i32,
    pub title:        String,
    pub show_fps:     bool,
    pub noise_amount: f32,

}

impl Settings {
    pub fn new() -> Result<Self, ConfigError>{

        let s = Config::builder()
            .add_source(File::with_name("./config.toml"))
            .build()?;
        println!("{:?}", s);
        s.try_deserialize()
    }
}