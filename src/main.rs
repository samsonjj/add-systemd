use std::path::PathBuf;
use toml;

use colored::Colorize;
use std::fs::File;
use std::io::Write;

use input::CommandLineReader;
use systemd::{Service, Systemd};

use crate::systemd::Restart;

mod input;
mod systemd;

fn main() -> std::io::Result<()> {
    let mut reader = CommandLineReader::new();

    let title = reader.ask("Title: ");
    let user = reader.ask("User: ");
    let start_command = reader.ask("Write the shell command: ");
    let restart = reader.options::<Restart>("Behavior on restart");

    let filename = format!("{}.service", title);

    let systemd = Systemd {
        Service: Service {
            ExecStart: start_command,
            User: user,
            Restart: restart,
        },
        ..Default::default()
    };
    let target_path = PathBuf::from("/etc/systemd/system").join(filename);

    let toml = toml::to_string(&systemd).unwrap();

    println!("creating file: ");
    println!();
    println!("target_path = {:?}", target_path);
    println!("----------------------------------");
    println!("");
    println!("{}", &toml.green());

    let answer = reader.ask("continue? [y/n]");
    if answer == "y" {
        println!("creating file...");
        let mut file = File::create(&target_path)?;
        file.write(toml.as_bytes())?;
        println!("{}", toml);
        println!("done");
    } else {
        println!("skipped creating file");
    }

    Ok(())
}

/*
[Unit]
Description=Minecraft Server service
After=network.target

[Service]
User=gamemaster
ExecStart=/home/gamemaster/minecraft-server/start-server.sh
Restart=always

[Install]
WantedBy=multi-user.target
 */
