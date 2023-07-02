use color_eyre::eyre::Result;
use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Default)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub client_id: String,
    pub client_secret: String,
    pub totp: String,
}

#[derive(Debug, Deserialize)]
struct Field {
    label: String,
    value: Option<String>,
    totp: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Response {
    fields: Vec<Field>,
}

pub fn reddit_credentials(entry: &str) -> Result<Credentials> {
    let output = Command::new("op")
        .arg("item")
        .arg("get")
        .arg(entry)
        .arg("--format=json")
        .output()?;

    let items = std::str::from_utf8(&output.stdout)?;
    let response: Response = serde_json::from_str(items)?;

    let mut credentials = Credentials {
        ..Default::default()
    };

    for field in response.fields {
        match field.label.as_str() {
            "username" => credentials.username = field.value.unwrap_or_default(),
            "password" => credentials.password = field.value.unwrap_or_default(),
            "one-time password" => credentials.totp = field.totp.unwrap_or_default(),
            "client_id" => credentials.client_id = field.value.unwrap_or_default(),
            "client_secret" => credentials.client_secret = field.value.unwrap_or_default(),
            _ => {}
        }
    }

    Ok(credentials)
}
