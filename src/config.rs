use warmy::{Store, Res, StoreOpt, SimpleKey, toml::Toml};
use serde::Deserialize;
use druid::Data;
pub fn get_config() -> Result<Res<Config>, String> {
    let ctx = &mut ();
    let home_dir = if let Some(home_dir) = dirs::home_dir() {
        home_dir
    } else {
        return Err("Unable to find the home directory".to_string());
    };
    let opt = StoreOpt::default().set_root(home_dir);
    let mut store: Store<(), SimpleKey> =
        Store::new(opt).map_err(|e| format!("error establishing storage {}", e))?;

    let resource: Res<Config> = store
        .get_by(&SimpleKey::from_path("/.lights"), ctx, Toml)
        .map_err(|e| format!("failed to get config from store {}", e))?;
    Ok(resource)
}


#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub switches: Vec<Switch>,
    pub rabbit: RabbitConfig,
}
#[derive(Debug, Deserialize, Clone)]
pub struct RabbitConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Data, Clone)]
pub struct Switch {
    pub name: String,
    pub on_code: u32,
    pub off_code: u32,
}