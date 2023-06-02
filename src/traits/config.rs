use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, OpenOptions},
    io::{Write, self},
    path::PathBuf,
};

use log::*;
use serde::{de::DeserializeOwned, Serialize};

pub trait ConfigTriat
where
    Self: Serialize + DeserializeOwned + Default,
{
    const NAME: &'static str;

    fn path() -> PathBuf {
        dirs::config_dir()
            .unwrap()
            .join(env::var("CARGO_PKG_NAME").unwrap())
            .join(format!("{}.yml", Self::NAME))
    }

    fn load() -> Result<Self, Box<dyn Error>> {
        let path = Self::path();

        trace!("Reading config file at {:?}", Self::path());

        let config = if path.exists() {
            let s = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(e) => {
                    return Err(crate::error::Error::StringErr(format!(
                        "unable to read config file at `{path:?}`: {e}",
                    ))
                    .into())
                }
            };

            match serde_yaml::from_str(&s) {
                Ok(v) => v,
                Err(e) => {
                    return Err(crate::error::Error::StringErr(format!(
                        "error parsing config file at `{path:?}`: {e}"
                    ))
                    .into())
                }
            }
        } else {
            info!("No config file found at {:?}, using default", Self::path());
            Self::default()
        };

        trace!("Saving config file after load to {:?}", Self::path());
        config.save()?;
        Ok(config)
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let s = serde_yaml::to_string(&self).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(Self::path())?;
        file.write_all(s.as_bytes())?;

        Ok(())
    }

    fn to_map(&self) -> HashMap<String, String> {
        serde_yaml::from_value(serde_yaml::to_value(self).unwrap()).unwrap()
    }

    fn extend_map(&self, map: &mut HashMap<String, String>) {
        map.extend(self.to_map())
    }

    fn delete_config() -> Result<(), io::Error> {
        fs::remove_file(Self::path())
    }
}
