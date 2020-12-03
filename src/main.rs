use structopt::clap::AppSettings;
use structopt::StructOpt;

mod claims;
use claims::ClaimsCli;
mod ctl;
use ctl::CtlCli;
mod keys;
use keys::KeysCli;
mod par;
use par::ParCli;
mod reg;
use reg::RegCli;
mod util;

/// This renders appropriately with escape characters
const ASCII: &str = "
                               _____ _                 _    _____ _          _ _ 
                              / ____| |               | |  / ____| |        | | |
 __      ____ _ ___ _ __ ___ | |    | | ___  _   _  __| | | (___ | |__   ___| | |
 \\ \\ /\\ / / _` / __| '_ ` _ \\| |    | |/ _ \\| | | |/ _` |  \\___ \\| '_ \\ / _ \\ | |
  \\ V  V / (_| \\__ \\ | | | | | |____| | (_) | |_| | (_| |  ____) | | | |  __/ | |
   \\_/\\_/ \\__,_|___/_| |_| |_|\\_____|_|\\___/ \\__,_|\\__,_| |_____/|_| |_|\\___|_|_|

A single CLI to handle all of your wasmCloud tooling needs
";

#[derive(Debug, Clone, StructOpt)]
#[structopt(global_settings(&[AppSettings::ColoredHelp, AppSettings::VersionlessSubcommands, AppSettings::DisableHelpSubcommand]),
            name = "wash",
            about = ASCII)]
struct Cli {
    #[structopt(flatten)]
    command: CliCommand,
}

#[derive(Debug, Clone, StructOpt)]
enum CliCommand {
    /// Generate and manage JWTs for wasmCloud Actors
    #[structopt(name = "claims")]
    Claims(ClaimsCli),
    /// Generate and manage wasmCloud keys
    #[structopt(name = "keys")]
    Keys(KeysCli),
    /// Interact with a wasmCloud control interface
    #[structopt(name = "ctl")]
    Ctl(CtlCli),
    /// Create, inspect, and modify capability provider archive files
    #[structopt(name = "par")]
    Par(ParCli),
    /// Interact with OCI compliant registries
    #[structopt(name = "reg")]
    Reg(RegCli),
}

fn main() {
    let cli = Cli::from_args();
    env_logger::init();

    let res = match cli.command {
        CliCommand::Keys(keyscli) => keys::handle_command(keyscli),
        CliCommand::Ctl(ctlcli) => ctl::handle_command(ctlcli),
        CliCommand::Claims(claimscli) => claims::handle_command(claimscli),
        CliCommand::Par(parcli) => par::handle_command(parcli),
        CliCommand::Reg(regcli) => reg::handle_command(regcli),
    };

    match res {
        Ok(_v) => (),
        Err(e) => println!("Error: {}", e),
    }
}
