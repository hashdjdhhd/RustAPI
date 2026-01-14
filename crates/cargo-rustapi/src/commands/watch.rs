//! Watch command for development

use anyhow::Result;
use clap::Args;
use console::style;
use tokio::process::Command;

#[derive(Args, Debug)]
pub struct WatchArgs {
    /// Command to run (default: "run")
    #[arg(short, long, default_value = "run")]
    pub command: String,

    /// Clear screen before each run
    #[arg(short = 'c', long)]
    pub clear: bool,
}

pub async fn watch(args: WatchArgs) -> Result<()> {
    println!("{}", style("Starting watch mode...").bold());

    // Check if cargo-watch is installed
    let version_check = Command::new("cargo")
        .args(["watch", "--version"])
        .output()
        .await;

    if version_check.is_err() || !version_check.unwrap().status.success() {
        println!(
            "{}",
            style("cargo-watch is not installed. Installing...").yellow()
        );
        Command::new("cargo")
            .args(["install", "cargo-watch"])
            .status()
            .await?;
    }

    let mut cmd = Command::new("cargo");
    cmd.arg("watch");

    if args.clear {
        cmd.arg("-c");
    }

    cmd.arg("-x").arg(&args.command);

    // Ignore common directories to improve performance
    cmd.args(["-i", ".git", "-i", "target", "-i", "node_modules"]);

    cmd.spawn()?.wait().await?;

    Ok(())
}
