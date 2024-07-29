use std::{process::Command, str::FromStr};

use anyhow::Error;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    com: Com,
}

#[derive(Debug, Clone)]
enum Com {
    Yazi,
    Git,
}

impl FromStr for Com {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "yazi" => Ok(Com::Yazi),
            "git" => Ok(Com::Git),
            _ => Err(anyhow::anyhow!("Invalid command")),
        }
    }
}

fn main() {
    let arg = Args::parse();
    // println!("{:?}", arg.com);
    // use yazi open target path
    let mut path = match arg.com {
        Com::Yazi => Command::new("yazi")
            .arg("--chooser-file=/dev/stdout")
            .spawn()
            .unwrap(),
        Com::Git => Command::new("lazygit").spawn().unwrap(),
    };

    let _ = path.wait().unwrap();
    println!("{:?}", path);
}
