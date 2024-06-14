use colored::*;
use futures_channel::mpsc;
use futures_util::StreamExt;
use log::info;
use crate::action::{ActionMessage, Action}; // Importar ActionMessage y Action

pub async fn run(mut rx: mpsc::Receiver<ActionMessage>) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(action) = rx.next().await {
        match action {
            ActionMessage::NewSession(session_id) => {
                info!("New session started with ID: {}", session_id);
                println!("{}", format!("New session started with ID: {}", session_id).blue());
            }
            ActionMessage::AddAction((_, Action::Command { command })) => {
                info!("Executing command: {}", command);
                println!("{}", format!("Executing command: {}", command).blue());
                if let Err(e) = execute_command(&command) {
                    eprintln!("{}", format!("Command execution failed: {}", e).red());
                }
            }
            ActionMessage::AddAction((_, Action::Read { path })) => {
                info!("Reading file: {}", path.display());
                println!("{}", format!("Reading file: {}", path.display()).blue());
                // Handle file read logic here
            }
            ActionMessage::AddAction((_, Action::Write { path, content })) => {
                info!("Writing to file: {}", path.display());
                println!("{}", format!("Writing to file: {}", path.display()).blue());
                // Handle file write logic here
            }
            ActionMessage::ConfirmAction(_) => {
                info!("Confirm action received");
                println!("{}", "Confirm action received".blue());
                // Handle confirm action logic here
            }
            ActionMessage::StopAction(_) => {
                info!("Stop action received");
                println!("{}", "Stop action received".blue());
                // Handle stop action logic here
            }
        }
    }

    Ok(())
}

fn execute_command(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()?;

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout).blue());
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr).red());
    }

    Ok(())
}
