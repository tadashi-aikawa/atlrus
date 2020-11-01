mod app;
mod external;

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use env_logger::Env;
use serde::Deserialize;
use structopt::StructOpt;

use crate::app::bitbucket::create_groups::{do_create_groups, CreateGroupsOperation};
use crate::app::bitbucket::invite_members::{do_invite_members, InviteMembersOperation};
use crate::app::bitbucket::add_group_members::{AddGroupMembersOperation, do_add_group_members};

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

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
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let args: Args = Args::from_args();
    let json_str = fs::read_to_string(&args.input)?;
    let operation = serde_json::from_str::<Operation>(&json_str)?;

    if let Some(op) = operation.create_groups {
        info!(">>>>>>>>>> Create groups");
        do_create_groups(&op).await
    }

    if let Some(op) = operation.invite_members {
        info!(">>>>>>>>>> Invite members");
        do_invite_members(&op).await
    }

    if let Some(op) = operation.add_group_members {
        info!(">>>>>>>>>> Add members to groups");
        do_add_group_members(&op).await
    }

    Ok(())
}
