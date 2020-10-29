use std::collections::HashMap;
use std::env;

use anyhow::Result;
use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

const URL: &str = "https://api.bitbucket.org/1.0";

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
    static ref USER_NAME: String =
        env::var("ATLRUS_USER_NAME").expect("You must specify ATLRUS_USER_NAME");
    static ref APP_PASSWORD: String =
        env::var("ATLRUS_APP_PASSWORD").expect("You must specify ATLRUS_APP_PASSWORD");
}

/// Actually.. there are more properties.
#[derive(Deserialize, Debug)]
pub struct PostGroupsResponse {
    pub name: String,
    pub slug: String,
}

#[derive(Error, Debug)]
pub enum PostGroupError {
    #[error("group already exists")]
    GroupAlreadyExists,
    #[error("Client error: {status:?}.  detail: {detail:?}")]
    ClientError { status: StatusCode, detail: String },
    #[error("Server error: {status:?}.  detail: {detail:?}")]
    ServerError { status: StatusCode, detail: String },
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

/// Create a group in specified workspace.
pub async fn post_groups(
    workspaces_uuid: &str,
    group_name: &str,
) -> Result<PostGroupsResponse, PostGroupError> {
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
        StatusCode::BAD_REQUEST => Err(PostGroupError::GroupAlreadyExists),
        s if s.is_client_error() => Err(PostGroupError::ClientError {
            status: s,
            detail: res.text().await?,
        }),
        s if s.is_server_error() => Err(PostGroupError::ServerError {
            status: s,
            detail: res.text().await?,
        }),
        _ => Ok(res.json::<PostGroupsResponse>().await?),
    }
}

#[derive(Deserialize, Debug)]
pub struct PutGroupMembersResponse {
    pub display_name: String,
    /// Either uuid or email
    pub uuid: String,
    pub account_id: String,
    pub nickname: String,
    pub avatar: String,
    pub is_team: bool,
    pub is_staff: bool,
    pub resource_uri: String,
}

/// Add a member to a group
pub async fn put_group_member(
    workspaces_uuid: &str,
    group_slug: &str,
    uuid: &str,
) -> Result<PutGroupMembersResponse> {
    let url = format!(
        "{base_url}/groups/{workspace}/{group_slug}/members/{uuid}",
        base_url = URL,
        workspace = workspaces_uuid,
        group_slug = group_slug,
        uuid = uuid,
    );

    let res = CLIENT
        .put(&url)
        .basic_auth(USER_NAME.to_string(), Some(APP_PASSWORD.to_string()))
        .send()
        .await?;

    match res.status() {
        s if s.is_client_error() => bail!("Client error: {}. detail: {}", s, res.text().await?),
        s if s.is_server_error() => bail!("Server error: {}. detail: {}", s, res.text().await?),
        _ => Ok(res.json::<PutGroupMembersResponse>().await?),
    }
}

/// Actually.. there are more properties.
#[derive(Deserialize, Debug)]
pub struct PostInvitationsResponse {
    pub email: String,
}

pub async fn post_invitations(
    repository: &str,
    permission: &str,
    email: &str,
) -> Result<PostInvitationsResponse> {
    let url = format!(
        "{base_url}/invitations/{repository}",
        base_url = URL,
        repository = repository,
    );

    let mut params = HashMap::new();
    params.insert("permission", permission);
    params.insert("email", email);

    let res = CLIENT
        .post(&url)
        .basic_auth(USER_NAME.to_string(), Some(APP_PASSWORD.to_string()))
        .form(&params)
        .send()
        .await?;

    match res.status() {
        s if s.is_client_error() => bail!("Client error: {}. detail: {}", s, res.text().await?),
        s if s.is_server_error() => bail!("Server error: {}. detail: {}", s, res.text().await?),
        _ => Ok(res.json::<PostInvitationsResponse>().await?),
    }
}
