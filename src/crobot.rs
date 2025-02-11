use color_eyre::Result;
use eyre::Context as _;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use serde::Serialize;
use std::env::var;

use crate::APP_USER_AGENT;

#[derive(Debug, Clone, Serialize)]
pub struct CrobotWebook {
    pub discord_username: String,
}

impl CrobotWebook {
    pub fn new(discord_username: String) -> Self {
        CrobotWebook { discord_username }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NocoWebook {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: Rows,
}

#[derive(Debug, Clone, Serialize)]
pub struct Rows {
    #[serde(rename = "rows")]
    pub users: Vec<CrobotWebook>,
}

impl NocoWebook {
    pub fn new(users: Vec<CrobotWebook>) -> Self {
        NocoWebook {
            type_: "records.after.insert".to_string(),
            data: Rows { users },
        }
    }
}

pub fn send_webhook(updates: Vec<Option<CrobotWebook>>) -> Result<()> {
    let Some((hook, secret)) = crobot_webhook()? else {
        return Ok(());
    };

    let users: Vec<_> = updates.into_iter().filter_map(|a| a).collect();
    if users.len() == 0 {
        return Ok(());
    }

    let body = NocoWebook::new(users);

    let client = reqwest::blocking::Client::new();

    client
        .post(hook)
        .body(serde_json::to_string(&body).with_context(|| "Serializing webhook body")?)
        .header(USER_AGENT, APP_USER_AGENT)
        .header(CONTENT_TYPE, "application/json")
        .header("x-cssa-secret", secret)
        .send()
        .with_context(|| "Updating crobot discord users")?;

    Ok(())
}

fn crobot_webhook() -> Result<Option<(String, String)>> {
    if var("CROBOT_DISABLE").is_ok() {
        return Ok(None);
    }

    var("UPDATE_WEBOOK")
        .with_context(|| "Fetching update webhook")
        .and_then(|hook| {
            Ok((
                hook,
                var("CSSA_SECRET").with_context(|| "Fetching cssa secret")?,
            ))
        })
        .and_then(|v| Ok(Some(v)))
}
