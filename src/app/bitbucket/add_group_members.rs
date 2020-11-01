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
        for email in group.emails.iter() {
            match put_group_member(&op.workspace_uuid, &group.slug, email).await {
                Ok(_) => info!("🟢 Add {} to {}.", &email, &group.slug),
                Err(err) => match err {
                    AlreadyExists { .. } => {
                        info!("🟤 `{}` already exists in {}.", &email, &group.slug)
                    }
                    NotFound { .. } => warn!(
                        "🟡 At least either `{}` or `{}` is not found.",
                        &group.slug, &email
                    ),
                    _ => log::error!("🔴 Fail to add {} to {}.", &email, &group.slug),
                },
            }
        }
    }
}
