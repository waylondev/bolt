use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bolt")]
#[command(author = "Waylon Wang")]
#[command(version = "0.1.0")]
#[command(about = "高性能负载测试工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "debug")]
    Debug {
        #[arg(short, long)]
        url: String,
        #[arg(short, long, default_value = "GET")]
        method: String,
    },
    #[command(name = "load-test")]
    LoadTest {
        #[arg(short, long)]
        url: String,
        #[arg(short, long, default_value = "10")]
        concurrent: usize,
        #[arg(short, long, default_value = "30")]
        duration: u64,
    },
    #[command(name = "version")]
    Version,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Debug { url, method } => {
            let handler = bolt::CliHandler::new()?;
            handler.handle_debug(&url, &method).await?;
        }
        Commands::LoadTest {
            url,
            concurrent,
            duration,
        } => {
            let handler = bolt::CliHandler::new()?;
            handler
                .handle_load_test(&url, "GET", concurrent, duration)
                .await?;
        }
        Commands::Version => {
            println!("Bolt v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
