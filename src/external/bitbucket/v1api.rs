use std::env;

use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

const URL: &str = "https://api.bitbucket.org/1.0";

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref USER_NAME: String =
        env::var("ATLRUST_USER_NAME").expect("You must specify ATLRUST_USER_NAME");
    static ref APP_PASSWORD: String =
        env::var("ATLRUST_APP_PASSWORD").expect("You must specify ATLRUST_APP_PASSWORD");
}

/// Actually.. there are more properties.
#[derive(Deserialize, Debug)]
pub struct CreateGroupsResponse {
    pub name: String,
    pub slug: String,
}

/// Create a group in specified workspace.
pub async fn create_group(workspaces_uuid: &str, group_name: &str) -> Result<CreateGroupsResponse> {
    let url = format!(
        "{base_url}/groups/{workspace}",
        base_url = URL,
        workspace = workspaces_uuid,
    );

    let mut params = HashMap::new();
    params.insert("name", group_name);

    let res = CLIENT
        .post(&url)
        .basic_auth(USER_NAME.to_string(), Some(APP_PASSWORD.to_string()))
        .form(&params)
        .send()
        .await?;

    match res.status() {
        s if s.is_client_error() => Err(anyhow!(
            "Client error: {}. detail: {}",
            s,
            res.text().await?
        )),
        s if s.is_server_error() => Err(anyhow!(
            "Server error: {}. detail: {}",
            s,
            res.text().await?
        )),
        _ => Ok(res.json::<CreateGroupsResponse>().await?),
    }
}
