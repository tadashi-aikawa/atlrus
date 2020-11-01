use serde::Deserialize;

use crate::external::bitbucket::v1api::post_groups;
use crate::external::bitbucket::v1api::PostGroupError::GroupAlreadyExists;

// TODO: -> domain model
#[derive(Deserialize, Debug)]
struct Group {
    slug: String,
    emails: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateGroupsOperation {
    workspace_uuid: String,
    group_names: Vec<String>,
}

pub async fn do_create_groups(op: &CreateGroupsOperation) {
    for group_name in op.group_names.iter() {
        match post_groups(&op.workspace_uuid, &group_name).await {
            Ok(group) => info!("🟢 Create a new group: {}.", group.name),
            Err(err) => match err {
                GroupAlreadyExists => info!("🟤 Group `{}` already exists.", group_name),
                _ => log::error!("🔴 Fail to create a new group: {}.  {}", group_name, err),
            },
        }
    }
}
