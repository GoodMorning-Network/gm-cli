use std::{env, error::Error};

use log::*;
use reqwest::header::USER_AGENT;
use serde::{de::DeserializeOwned, Serialize};

pub fn post<R: DeserializeOwned, T: Serialize + Sized>(
    url: &str,
    body: T,
) -> Result<R, Box<dyn Error>> {
    info!("Sending request");
    let res = reqwest::blocking::Client::new()
        .post(url)
        .header(
            USER_AGENT,
            &format!("gm-cli {}", env::var("CARGO_PKG_VERSION").unwrap()),
        )
        .json(&body)
        .send();
    let res = match res {
        Ok(res) => res,
        Err(e) => {
            error!("Error sending request to `{url}`");
            return Err(crate::error::Error::StringErr(e.to_string()).into());
        }
    };
    trace!("Response recieved, deserializing");
    match res.json() {
        Ok(out) => Ok(out),
        Err(e) => {
            error!("Deserialization failed");
            Err(crate::error::Error::StringErr(e.to_string()).into())
        }
    }
}