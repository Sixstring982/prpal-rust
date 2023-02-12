mod graph;
mod logging;
mod pulls;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "prpal")]
#[command(about = "Pull request helper", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Graph {
        #[arg(long)]
        auth_token: String,
        #[arg(long)]
        repo: String,
        #[arg(long)]
        author: Option<String>,
        #[arg(long, value_enum, default_value_t = logging::LogLevel::Off)]
        log_level: logging::LogLevel,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let run_result = match args.command {
        Commands::Graph {
            auth_token,
            repo,
            author,
            log_level,
        } => run_graph_command(auth_token, repo, author, log_level),
    }.await;

    if run_result.is_err() {
        eprintln!("{:?}", run_result)
    }
}

async fn run_graph_command(
    auth_token: String,
    repo: String,
    author: Option<String>,
    log_level: logging::LogLevel,
) -> Result<(), Box<dyn std::error::Error>> {
    use pulls::list_pull_requests::*;

    logging::init(log_level)?;
    let response = list_pull_requests(Request { auth_token, author, repo }).await?;
    let output = graph::render_graph(graph::OutputFormat::Mermaid, &response.pull_requests);

    println!("{}", output);

    Ok(())
}
