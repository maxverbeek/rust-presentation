use clap::{Parser, Subcommand};

use dotenv::dotenv;
use officetracker::{MacStorage, MacUser};

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        slack: String,

        #[arg(short, long)]
        mac_addr: String,
    },

    List,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var must be set");

    let storage = MacStorage::new(&url)
        .await
        .expect("Cannot connect to database");

    match args.command {
        Commands::List => {
            for user in storage.list_users().await.expect("Could not fetch users") {
                println!("U: {}, M: {}", user.user, user.mac);
            }
        }

        Commands::Add { slack, mac_addr } => {
            storage
                .add_macuser(&MacUser {
                    user: slack,
                    mac: mac_addr,
                })
                .await
                .expect("Failed to add user to database");
        }
    }
}
