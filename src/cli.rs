use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about= None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    /// Pick a context from the kube config - Default command if none is specified
    #[command(name = "ctx")]
    PickContext,
    /// Pick a namespace for the current context
    #[command(name = "ns")]
    PickNamespace,
}
