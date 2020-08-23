#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;
use structopt::StructOpt;

use external::bitbucket;

mod external;

#[derive(Deserialize, Debug)]
struct Group {
    slug: String,
    emails: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct CreateGroupsOperation {
    workspace_uuid: String,
    group_names: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct InviteMembersOperation {
    /// Ex: tadashi-aikawa/x-viewer
    repository: String,
    /// Ex: read, write
    permission: String,
    emails: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct AddGroupMembersOperation {
    workspace_uuid: String,
    groups: Vec<Group>,
}

#[derive(Deserialize, Debug)]
struct Operation {
    create_groups: Option<CreateGroupsOperation>,
    invite_members: Option<InviteMembersOperation>,
    add_group_members: Option<AddGroupMembersOperation>,
}

#[derive(Debug, StructOpt)]
struct Args {
    /// Input parameter json file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Args::from_args();
    let json_str = fs::read_to_string(&args.input)?;
    let operation = serde_json::from_str::<Operation>(&json_str)?;

    if let Some(op) = operation.create_groups {
        do_create_groups(&op).await
    }

    if let Some(op) = operation.invite_members {
        do_invite_members(&op).await
    }

    if let Some(op) = operation.add_group_members {
        do_add_group_members(&op).await
    }

    Ok(())
}

async fn do_create_groups(op: &CreateGroupsOperation) {
    for group_name in op.group_names.iter() {
        match bitbucket::v1api::post_groups(&op.workspace_uuid, &group_name).await {
            Ok(group) => println!("Create a new group, {}!!", group.name),
            Err(err) => {
                println!("Fail to create a new group, {}..", group_name);
                println!("{}", err)
            }
        }
    }
}

async fn do_invite_members(op: &InviteMembersOperation) {
    for email in op.emails.iter() {
        match bitbucket::v1api::post_invitations(&op.repository, &op.permission, &email).await {
            Ok(_) => println!("Invite {}!!", &email),
            Err(err) => {
                println!("Fail to invite {}..", &email);
                println!("{}", err)
            }
        }
    }
}

async fn do_add_group_members(op: &AddGroupMembersOperation) {
    for group in op.groups.iter() {
        for email in group.emails.iter() {
            match bitbucket::v1api::put_group_member(&op.workspace_uuid, &group.slug, email).await {
                Ok(_) => println!("Add {} to {}!!", &email, &group.slug),
                Err(err) => {
                    println!("Fail to add {} to {}..", &email, &group.slug);
                    println!("{}", err)
                }
            }
        }
    }
}
