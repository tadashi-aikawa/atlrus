#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;

use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;
use structopt::StructOpt;

use external::bitbucket;

mod external;

#[derive(Deserialize, Debug)]
struct Operation {
    workspace_uuid: String,
    groups_to_add: Vec<String>,
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

    for group_name in operation.groups_to_add.iter() {
        match bitbucket::v1api::create_group(&operation.workspace_uuid, &group_name).await {
            Ok(group) => println!("Create a new group, {}!!", group.name),
            Err(err) => {
                println!("Fail to create a new group, {}..", group_name);
                println!("{}", err)
            }
        }
    }

    Ok(())
}
