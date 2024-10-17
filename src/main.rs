use std::fmt::Display;

use anyhow::anyhow;
use clap::Parser;
use inquire::{InquireError, Select};
use k8s_openapi::api::core::v1::Namespace;
use kube::{
    api::ListParams,
    client::scope::Cluster,
    config::{Kubeconfig, NamedContext},
    Client, Config,
};
use kubectl_copi::cli::{Args, Command};
use log::debug;

struct Context<'a>(&'a NamedContext);

impl<'a> Display for Context<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cluster = &self
            .0
            .context
            .as_ref()
            .map_or_else(|| "N/A".to_owned(), |i| i.cluster.to_owned());
        let namespace = &self
            .0
            .context
            .as_ref()
            .map(|i| i.namespace.to_owned())
            .flatten()
            .map_or_else(|| "N/A".to_owned(), |i| i);
        write!(
            f,
            "{} (cluster: {}, namespace: {})",
            self.0.name, cluster, namespace
        )
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Args::parse();

    match args.command {
        Some(Command::PickNamespace) => pick_namespace().await,
        _ => pick_context(),
    }
}

fn pick_context() -> anyhow::Result<()> {
    let config = Kubeconfig::read()?;
    let current_context = config.current_context.unwrap_or_default();
    let context_list: Vec<Context> = config.contexts.iter().map(|i| Context(i)).collect();
    let current_context_index = context_list
        .iter()
        .enumerate()
        .find(|(_, context)| context.0.name == current_context)
        .map_or(0, |i| i.0);

    let context: Result<Context, InquireError> = Select::new("Context: ", context_list)
        .with_vim_mode(true)
        .with_starting_cursor(current_context_index)
        .prompt();

    match context {
        Ok(choice) => {
            if choice.0.name != current_context {
                std::process::Command::new("kubectl")
                    .args(vec!["config", "use-context", choice.0.name.as_str()])
                    .status()?;
            }
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

async fn pick_namespace() -> anyhow::Result<()> {
    let client = match get_client().await {
        Err(kube::Error::Api(err)) if err.code == 401 => {
            return Err(anyhow!("Unauthorized - are you logged in on the cluster?"));
        }
        Err(e) => return Err(e.into()),
        Ok(client) => client,
    };

    let namespaces = client
        .list::<Namespace>(&ListParams::default(), &Cluster)
        .await?;

    let namespaces_list: Vec<String> = namespaces
        .iter()
        .filter_map(|i| i.metadata.name.as_ref().map(|n| n.to_owned()))
        .collect();

    let config = Config::infer().await?;
    let current_namespace = config.default_namespace;
    let current_namespace_index = namespaces_list
        .iter()
        .enumerate()
        .find(|(_, ns)| **ns == current_namespace)
        .map_or(0, |i| i.0);

    let namespace: Result<String, InquireError> = Select::new("Namespace: ", namespaces_list)
        .with_vim_mode(true)
        .with_starting_cursor(current_namespace_index)
        .prompt();

    match namespace {
        Ok(choice) => {
            if choice != current_namespace {
                std::process::Command::new("kubectl")
                    .args(vec![
                        "config",
                        "set-context",
                        "--current",
                        "--namespace",
                        choice.as_str(),
                    ])
                    .status()?;
            }
            Ok(())
        }
        Err(err) => Err(err.into()),
    }
}

async fn get_client() -> Result<Client, kube::Error> {
    let client = Client::try_default()
        .await
        .expect("failed to init default kubernetes client");

    debug!("connecting to cluster...");

    match client.apiserver_version().await {
        Ok(version_info) => {
            debug!("connected to cluster - version info: {:?}", version_info);
            Ok(client)
        }
        Err(e) => Err(e),
    }
}
