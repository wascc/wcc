extern crate control_interface;
use control_interface::*;
use std::time::Duration;
use structopt::StructOpt;
use term_table::row::Row;
use term_table::table_cell::*;
use term_table::{Table, TableStyle};
use tokio::runtime::Runtime;

use crate::util::convert_error;
type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

//TODO(brooksmtownsend): If theres a deadline that elapses, suggest specifying a namespace

#[derive(Debug, Clone, StructOpt)]
pub struct CtlCli {
    #[structopt(flatten)]
    command: CtlCliCommand,
}

#[derive(Debug, Clone, StructOpt)]
pub struct ConnectionOpts {
    /// RPC Host for connection, defaults to 0.0.0.0 for local nats
    #[structopt(short = "r", long = "rpc-host", default_value = "0.0.0.0")]
    rpc_host: String,

    /// RPC Port for connections, defaults to 4222 for local nats
    #[structopt(short = "p", long = "rpc-port", default_value = "4222")]
    rpc_port: String,

    /// Namespace prefix for wasmCloud command interface
    #[structopt(short = "n", long = "ns-prefix")]
    ns_prefix: Option<String>,

    /// Timeout length for RPC, defaults to 1 second
    #[structopt(short = "t", long = "timeout", default_value = "1")]
    timeout: u64,
}

#[derive(Debug, Clone, StructOpt)]
pub enum CtlCliCommand {
    /// Retrieves information about the lattice
    #[structopt(name = "get")]
    Get(GetCommand),

    /// Start an actor or a provider
    #[structopt(name = "start")]
    Start(StartCommand),

    /// Stop an actor or a provider
    #[structopt(name = "stop")]
    Stop(StopCommand),
}

#[derive(Debug, Clone, StructOpt)]
pub enum GetCommand {
    /// Query lattice for running hosts
    #[structopt(name = "hosts")]
    Hosts(GetHostsCommand),

    /// Query lattice for the inventory of a host
    #[structopt(name = "inventory")]
    HostInventory(GetHostInventoryCommand),

    /// Query lattice for claims present in the lattice
    #[structopt(name = "claims")]
    Claims(GetClaimsCommand),
}

#[derive(Debug, Clone, StructOpt)]
pub enum StartCommand {
    /// Launch an actor in a host
    #[structopt(name = "actor")]
    Actor(ActorCommand),

    /// Launch a provider in a host
    #[structopt(name = "provider")]
    Provider(StartProviderCommand),
}

#[derive(Debug, Clone, StructOpt)]
pub enum StopCommand {
    /// Stop an actor running in a host
    #[structopt(name = "actor")]
    Actor(ActorCommand),

    /// Stop a provider running in a host
    #[structopt(name = "provider")]
    Provider(StopProviderCommand),
}

#[derive(Debug, Clone, StructOpt)]
pub struct GetHostsCommand {
    #[structopt(flatten)]
    opts: ConnectionOpts,
}

#[derive(Debug, Clone, StructOpt)]
pub struct GetHostInventoryCommand {
    #[structopt(flatten)]
    opts: ConnectionOpts,

    /// Id of host
    #[structopt(name = "host-id")]
    host_id: String,
}

#[derive(Debug, Clone, StructOpt)]
pub struct ActorCommand {
    #[structopt(flatten)]
    opts: ConnectionOpts,

    /// Id of host
    #[structopt(name = "host-id")]
    host_id: String,

    /// Actor reference, e.g. the OCI URL for the actor
    #[structopt(name = "actor-ref")]
    actor_ref: String,
}

#[derive(Debug, Clone, StructOpt)]
pub struct StartProviderCommand {
    #[structopt(flatten)]
    opts: ConnectionOpts,

    /// Id of host
    #[structopt(name = "host-id")]
    host_id: String,

    /// Provider reference, e.g. the OCI URL for the provider
    #[structopt(name = "provider-ref")]
    provider_ref: String,

    /// Link name of provider
    #[structopt(short = "l", long = "link-name")]
    link_name: Option<String>,
}

#[derive(Debug, Clone, StructOpt)]
pub struct StopProviderCommand {
    #[structopt(flatten)]
    opts: ConnectionOpts,

    /// Id of host
    #[structopt(name = "host-id")]
    host_id: String,

    /// Provider reference, e.g. the OCI URL for the provider
    #[structopt(name = "provider-ref")]
    provider_ref: String,

    /// Link name of provider
    #[structopt(name = "link-name")]
    link_name: String,

    /// Capability contract Id of provider
    #[structopt(name = "contract-id")]
    contract_id: String,
}

#[derive(Debug, Clone, StructOpt)]
pub struct GetClaimsCommand {
    #[structopt(flatten)]
    opts: ConnectionOpts,
}

pub fn handle_command(cli: CtlCli) -> Result<()> {
    let mut rt = Runtime::new().unwrap();
    // Since match arms are incompatible types, I can't surround this with a block_on
    use CtlCliCommand::*;
    match cli.command {
        Get(GetCommand::Hosts(cmd)) => {
            let ns = cmd.opts.ns_prefix.clone();
            let hosts = rt.block_on(get_hosts(cmd))?;
            display_hosts(hosts, ns);
        }
        Get(GetCommand::HostInventory(cmd)) => {
            let inv = rt.block_on(get_host_inventory(cmd))?;
            display_host_inventory(inv);
        }
        Get(GetCommand::Claims(cmd)) => {
            let ns = cmd.opts.ns_prefix.clone();
            let claims = rt.block_on(get_claims(cmd))?;
            display_claims(claims, ns);
        }
        Start(StartCommand::Actor(cmd)) => {
            rt.block_on(start_actor(cmd))?;
        }
        Start(StartCommand::Provider(cmd)) => {
            rt.block_on(start_provider(cmd))?;
        }
        Stop(StopCommand::Actor(cmd)) => {
            rt.block_on(stop_actor(cmd))?;
        }
        Stop(StopCommand::Provider(cmd)) => {
            rt.block_on(stop_provider(cmd))?;
        }
    };
    Ok(())
}

pub async fn new_ctl_client(
    host: &str,
    port: &str,
    ns_prefix: Option<String>,
    timeout: Duration,
) -> Result<Client> {
    let nc = nats::asynk::connect(&format!("{}:{}", host, port)).await?;
    Ok(Client::new(nc, ns_prefix, timeout))
}

async fn client_from_opts(opts: ConnectionOpts) -> Result<Client> {
    new_ctl_client(
        &opts.rpc_host,
        &opts.rpc_port,
        opts.ns_prefix,
        Duration::from_secs(opts.timeout),
    )
    .await
}

pub async fn get_hosts(cmd: GetHostsCommand) -> Result<Vec<Host>> {
    let timeout = Duration::from_secs(cmd.opts.timeout);
    let client = client_from_opts(cmd.opts).await?;
    client.get_hosts(timeout).await.map_err(convert_error)
}

pub async fn get_host_inventory(cmd: GetHostInventoryCommand) -> Result<HostInventory> {
    let client = client_from_opts(cmd.opts).await?;
    client
        .get_host_inventory(&cmd.host_id)
        .await
        .map_err(convert_error)
}

pub async fn get_claims(cmd: GetClaimsCommand) -> Result<ClaimsList> {
    let client = client_from_opts(cmd.opts).await?;
    client.get_claims().await.map_err(convert_error)
}

pub async fn start_actor(cmd: ActorCommand) -> Result<StartActorAck> {
    let client = client_from_opts(cmd.opts).await?;
    client
        .start_actor(&cmd.host_id, &cmd.actor_ref)
        .await
        .map_err(convert_error)
}

pub async fn start_provider(cmd: StartProviderCommand) -> Result<StartProviderAck> {
    let client = client_from_opts(cmd.opts).await?;
    client
        .start_provider(&cmd.host_id, &cmd.provider_ref, cmd.link_name)
        .await
        .map_err(convert_error)
}

pub async fn stop_provider(cmd: StopProviderCommand) -> Result<StopProviderAck> {
    let client = client_from_opts(cmd.opts).await?;
    client
        .stop_provider(
            &cmd.host_id,
            &cmd.provider_ref,
            &cmd.link_name,
            &cmd.contract_id,
        )
        .await
        .map_err(convert_error)
}

pub async fn stop_actor(cmd: ActorCommand) -> Result<StopActorAck> {
    let client = client_from_opts(cmd.opts).await?;
    client
        .stop_actor(&cmd.host_id, &cmd.actor_ref)
        .await
        .map_err(convert_error)
}

/// Helper function to print a Host list to stdout as a table
fn display_hosts(hosts: Vec<Host>, ns_prefix: Option<String>) {
    let mut table = Table::new();
    table.max_column_width = 68;
    table.style = TableStyle::extended();

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!("Hosts - {}", ns_prefix.unwrap_or("".to_string())),
        2,
        Alignment::Center,
    )]));

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("Host ID", 1, Alignment::Left),
        TableCell::new_with_alignment("Uptime (seconds)", 1, Alignment::Right),
    ]));
    hosts.iter().for_each(|h| {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(h.id.clone(), 1, Alignment::Left),
            TableCell::new_with_alignment(format!("{}", h.uptime_seconds), 1, Alignment::Right),
        ]))
    });

    println!("{}", table.render());
}

/// Helper function to print a HostInventory to stdout as a table
fn display_host_inventory(inv: HostInventory) {
    let mut table = Table::new();
    table.max_column_width = 68;
    table.style = TableStyle::extended();

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!("Host Inventory - {}", inv.host_id),
        3,
        Alignment::Center,
    )]));

    if inv.labels.len() >= 1 {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "Labels",
            3,
            Alignment::Center,
        )]));
        inv.labels.iter().for_each(|(k, v)| {
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(k, 1, Alignment::Left),
                TableCell::new_with_alignment(v, 2, Alignment::Right),
            ]))
        });
    } else {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "No labels present",
            3,
            Alignment::Center,
        )]));
    }

    if inv.actors.len() >= 1 {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "Actors",
            3,
            Alignment::Center,
        )]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Actor ID", 1, Alignment::Left),
            TableCell::new_with_alignment("Image Reference", 2, Alignment::Right),
        ]));
        inv.actors.iter().for_each(|a| {
            let a = a.clone();
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(a.id, 1, Alignment::Left),
                TableCell::new_with_alignment(
                    a.image_ref.unwrap_or("N/A".to_string()),
                    2,
                    Alignment::Right,
                ),
            ]))
        });
    } else {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "No actors found",
            3,
            Alignment::Center,
        )]));
    }

    if inv.providers.len() >= 1 {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "Providers",
            3,
            Alignment::Center,
        )]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Provider ID", 1, Alignment::Left),
            TableCell::new_with_alignment("Link Name", 1, Alignment::Center),
            TableCell::new_with_alignment("Image Reference", 1, Alignment::Right),
        ]));
        inv.providers.iter().for_each(|p| {
            let p = p.clone();
            table.add_row(Row::new(vec![
                TableCell::new_with_alignment(p.id, 1, Alignment::Left),
                TableCell::new_with_alignment(p.link_name, 1, Alignment::Center),
                TableCell::new_with_alignment(
                    p.image_ref.unwrap_or("N/A".to_string()),
                    1,
                    Alignment::Right,
                ),
            ]))
        });
    } else {
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            "No providers found",
            3,
            Alignment::Center,
        )]));
    }

    println!("{}", table.render());
}

/// Helper function to print a ClaimsList to stdout as a table
fn display_claims(list: ClaimsList, ns_prefix: Option<String>) {
    let mut table = Table::new();
    table.max_column_width = 68;
    table.style = TableStyle::extended();

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!("Claims - {}", ns_prefix.unwrap_or("".to_string())),
        2,
        Alignment::Center,
    )]));

    list.claims.iter().for_each(|c| {
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Issuer", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.values.get("iss").unwrap_or(&"".to_string()),
                1,
                Alignment::Right,
            ),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Subject", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.values.get("sub").unwrap_or(&"".to_string()),
                1,
                Alignment::Right,
            ),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Capabilities", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.values.get("caps").unwrap_or(&"".to_string()),
                1,
                Alignment::Right,
            ),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Version", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.values.get("version").unwrap_or(&"".to_string()),
                1,
                Alignment::Right,
            ),
        ]));
        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Revision", 1, Alignment::Left),
            TableCell::new_with_alignment(
                c.values.get("rev").unwrap_or(&"".to_string()),
                1,
                Alignment::Right,
            ),
        ]));
        table.add_row(Row::new(vec![TableCell::new_with_alignment(
            format!(""),
            2,
            Alignment::Center,
        )]));
    });

    println!("{}", table.render());
}
