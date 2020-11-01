use serde::Deserialize;

use crate::external::bitbucket::v1api::post_invitations;

#[derive(Deserialize, Debug)]
pub struct InviteMembersOperation {
    /// Ex: tadashi-aikawa/x-viewer
    repository: String,
    /// Ex: read, write
    permission: String,
    emails: Vec<String>,
}

pub async fn do_invite_members(op: &InviteMembersOperation) {
    for email in op.emails.iter() {
        match post_invitations(&op.repository, &op.permission, &email).await {
            Ok(_) => info!("🟢 Invite {}.", &email),
            Err(err) => log::error!("🔴 Fail to invite {}.  {}", &email, err),
        }
    }
}
