use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// LeetCode problem ID (e.g., "1", "42")
    pub problem_id: String,

    /// Explicit language (default: cpp)
    #[arg(long, default_value = "cpp")]
    pub lang: String,

    /// Overwrite existing file
    #[arg(long)]
    pub force: bool,

    /// Compile & run immediately
    #[arg(long)]
    pub run: bool,

    /// Show inferred signature & testcases, don’t generate
    #[arg(long)]
    pub dry_run: bool,
    
    /// Disable cached problem data
    #[arg(long)]
    pub no_cache: bool,
}
