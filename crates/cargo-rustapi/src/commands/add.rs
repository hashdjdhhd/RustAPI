//! Add command to add features or dependencies

use anyhow::Result;
use clap::Args;
use tokio::process::Command;

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Crate name or RustAPI feature
    pub name: String,

    /// Add as a dev dependency
    #[arg(short, long)]
    pub dev: bool,
}

pub async fn add(args: AddArgs) -> Result<()> {
    println!("Adding dependency: {}", args.name);

    let mut cmd = Command::new("cargo");
    cmd.arg("add");

    if args.dev {
        cmd.arg("--dev");
    }

    cmd.arg(&args.name);

    let status = cmd.status().await?;

    if !status.success() {
        anyhow::bail!("Failed to add dependency");
    }

    Ok(())
}
