mod app;
mod external;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use env_logger::{fmt::Color, Env};
use serde::Deserialize;
use structopt::StructOpt;

use crate::app::bitbucket::add_group_members::{do_add_group_members, AddGroupMembersOperation};
use crate::app::bitbucket::create_groups::{do_create_groups, CreateGroupsOperation};
use crate::app::bitbucket::invite_members::{do_invite_members, InviteMembersOperation};
use log::Level;

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
    init_logger();

    let args: Args = Args::from_args();
    let json_str = fs::read_to_string(&args.input)?;
    let operation = serde_json::from_str::<Operation>(&json_str)?;

    if let Some(op) = operation.create_groups {
        info!("************************************************************************");
        info!("*                     👥 Create groups                                  ");
        info!("************************************************************************");
        do_create_groups(&op).await
    }

    if let Some(op) = operation.invite_members {
        info!("");
        info!("************************************************************************");
        info!("*                     📨 Invite members                                 ");
        info!("************************************************************************");
        do_invite_members(&op).await
    }

    if let Some(op) = operation.add_group_members {
        info!("");
        info!("************************************************************************");
        info!("*                     🍻 Add members to groups                          ");
        info!("************************************************************************");
        do_add_group_members(&op).await
    }

    Ok(())
}

fn init_logger() {
    env_logger::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let level_color = match record.level() {
                Level::Trace => Color::White,
                Level::Debug => Color::Blue,
                Level::Info => Color::Green,
                Level::Warn => Color::Yellow,
                Level::Error => Color::Red,
            };
            let mut level_style = buf.style();
            level_style.set_color(level_color);

            writeln!(
                buf,
                "[{timestamp}] {level}: {args}",
                timestamp = buf.timestamp(),
                level = level_style.value(record.level()),
                args = record.args()
            )
        })
        .init();
}
