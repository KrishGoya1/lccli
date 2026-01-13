mod cli;
mod fetch;
mod parse;
mod model;
mod codegen;
mod run;
mod error;

use clap::Parser;
use cli::args::Args;
use colored::*;
use fetch::leetcode::LeetCodeClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let cache = fetch::cache::Cache::new()?;

    // 1. Check cache
    let problem_data = if !args.no_cache {
        if let Ok(Some(data)) = cache.get(&args.problem_id) {
            println!("{}", format!("Loaded problem {} from cache.", args.problem_id).green());
            Some(data)
        } else {
            None
        }
    } else {
        None
    };

    let problem_data = match problem_data {
        Some(data) => data,
        None => {
            println!("{}", format!("Fetching problem {}...", args.problem_id).cyan());
            let client = LeetCodeClient::new();
            let data = client.resolve_problem_id(&args.problem_id).await?;
            
            cache.save(&args.problem_id, &data)?;
            println!("{}", "Saved to cache.".dimmed());
            data
        }
    };

    println!("Found problem: {} ({})", problem_data.title, problem_data.title_slug);
    println!("Difficulty: {}", problem_data.difficulty);
    
    // We will expand this later to do the rest of the flow
    Ok(())
}
