// src/main.rs
/*
 * Main executable for BeaconChain
 */

use clap::Parser;
use beaconchain::{Result, run};

/// Command-line arguments for BeaconChain
#[derive(Parser)]
#[command(version, about = "BeaconChain - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    /// Flag to enable or disable verbose output
    verbose: bool,
    
    /// Path to input file
    #[arg(short, long)]
    /// Input file path (optional)
    input: Option<String>,
    
    /// Path to output file
    #[arg(short, long)]
    /// Output file path (optional)
    output: Option<String>,
}

/// Main entry point for BeaconChain
fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Cli::parse();
    
    // Run BeaconChain with parsed arguments
    // This line is the core functionality of the program
    run(args.verbose, args.input, args.output)
}