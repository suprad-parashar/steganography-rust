use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    #[clap(value_parser)]
    pub file_path: String,
    #[clap(value_parser)]
    pub chunk_type: String,
    #[clap(value_parser)]
    pub message: String,
    #[clap(value_parser)]
    pub output_path: Option<String>,
}

#[derive(Args)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    pub file_path: String,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    pub file_path: String,
    #[clap(value_parser)]
    pub chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    #[clap(value_parser)]
    pub file_path: String,
}
