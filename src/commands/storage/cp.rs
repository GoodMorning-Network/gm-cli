use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use goodmorning_bindings::services::v1::{V1FromTo, V1Response};
use log::*;

use crate::config::AccountConfig;
use crate::error::Error as CError;
use crate::functions::{map_args, post, prompt_not_present};

const ARGS: &[&str] = &["from", "to"];

pub fn cp(mut map: HashMap<String, String>, args: Vec<String>) -> Result<String, Box<dyn Error>> {
    map_args(&mut map, ARGS, args)?;
    if !AccountConfig::is_loggedin_map(&map) {
        error!("You are not logged in");
        return Err(CError::StrErr("Not logged in").into());
    }

    prompt_not_present("From", "from", &mut map);
    prompt_not_present("To", "to", &mut map);

    let instance = map.get("instance").unwrap();
    let url = format!(
        "{}/api/storage/v1/{}",
        instance,
        if map.contains_key("overwrite") {
            "copy-overwrite"
        } else {
            "copy"
        }
    );

    let prefix = PathBuf::from(map.get("prefix").unwrap_or(&String::new()));
    let from = prefix.join(map.get("from").unwrap());
    let to = prefix.join(map.get("to").unwrap());
    let from_user = map.get("user").unwrap_or(map.get("id").unwrap());
    let token = map.get("token").unwrap().to_string();

    let body = V1FromTo {
        from: from.to_str().unwrap().to_string(),
        to: to.to_str().unwrap().to_string(),
        from_userid: from_user.parse()?,
        token,
    };

    let res = post(&url, body, map.contains_key("http"))?;

    match res {
        V1Response::Error { kind } => {
            error!("File not copied");
            return Err(CError::StringErr(kind.to_string()).into());
        }
        V1Response::Copied => {
            info!("Item copied successfully");
            info!("The copied path is `{}`", to.to_str().unwrap());
        }
        _ => unreachable!(),
    }

    Ok(String::from("Copied"))
}
