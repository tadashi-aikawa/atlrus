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
struct InviteOperation {
    /// Ex: tadashi-aikawa/x-viewer
    repository: String,
    /// Ex: read, write
    permission: String,
    emails: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct GroupsOperation {
    workspace_uuid: String,
    group_names: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Operation {
    create_groups: Option<GroupsOperation>,
    invite: Option<InviteOperation>,
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
        do_create_group(&op).await
    }

    if let Some(op) = operation.invite {
        do_invite(&op).await
    }

    Ok(())
}

async fn do_create_group(op: &GroupsOperation) {
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

async fn do_invite(op: &InviteOperation) {
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
