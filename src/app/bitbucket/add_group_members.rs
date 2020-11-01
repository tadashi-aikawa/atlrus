use serde::Deserialize;

use external::bitbucket::v1api::put_group_member;

use crate::external;
use crate::external::bitbucket::v1api::PutGroupMemberError::{AlreadyExists, NotFound};

// TODO: -> domain model
#[derive(Deserialize, Debug)]
struct Group {
    slug: String,
    emails: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct AddGroupMembersOperation {
    workspace_uuid: String,
    groups: Vec<Group>,
}

pub async fn do_add_group_members(op: &AddGroupMembersOperation) {
    for group in op.groups.iter() {
        info!("");
        info!("👥 {}          ", &group.slug);
        info!("--------------------------------------------------");
        for email in group.emails.iter() {
            match put_group_member(&op.workspace_uuid, &group.slug, email).await {
                Ok(_) => info!("🟢 👤 `{}` is added.", &email),
                Err(err) => match err {
                    AlreadyExists { .. } => info!("🟤 👤 `{}` already exists.", &email),
                    NotFound { .. } => warn!(
                        "🟡 👤 `{}` is not found (or group `{}` is not found).",
                        &email, &group.slug
                    ),
                    _ => log::error!("🔴 👤 `{}` can't be added for errors.", &email),
                },
            }
        }
    }
}
