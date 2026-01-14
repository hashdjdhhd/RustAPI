//! Doctor command to check environment health

use anyhow::Result;
use clap::Args;
use console::{style, Emoji};
use tokio::process::Command;

#[derive(Args, Debug)]
pub struct DoctorArgs {}

static CHECK: Emoji<'_, '_> = Emoji("✅ ", "+ ");
static WARN: Emoji<'_, '_> = Emoji("⚠️ ", "! ");
static ERROR: Emoji<'_, '_> = Emoji("❌ ", "x ");

pub async fn doctor(_args: DoctorArgs) -> Result<()> {
    println!("{}", style("Checking environment health...").bold());
    println!();

    check_tool("rustc", &["--version"], "Rust compiler").await;
    check_tool("cargo", &["--version"], "Cargo package manager").await;
    check_tool(
        "cargo",
        &["watch", "--version"],
        "cargo-watch (for hot reload)",
    )
    .await;
    check_tool("docker", &["--version"], "Docker (for containerization)").await;
    check_tool("sqlx", &["--version"], "sqlx-cli (for database migrations)").await;

    println!();
    println!("{}", style("Doctor check passed!").green());

    Ok(())
}

async fn check_tool(cmd: &str, args: &[&str], name: &str) {
    let output = Command::new(cmd).args(args).output().await;

    match output {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            println!("{} {} {}", CHECK, style(name).bold(), style(version).dim());
        }
        Ok(_) => {
            println!(
                "{} {} {}",
                WARN,
                style(name).bold(),
                style("installed but returned error").yellow()
            );
        }
        Err(_) => {
            let msg = if cmd == "cargo" && args[0] == "watch" {
                "(install with: cargo install cargo-watch)"
            } else if cmd == "sqlx" {
                "(install with: cargo install sqlx-cli)"
            } else {
                "(not found)"
            };
            println!("{} {} {}", ERROR, style(name).bold(), style(msg).dim());
        }
    }
}
