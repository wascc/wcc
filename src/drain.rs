use crate::util::{Output, OutputKind};
use fs::remove_file;
use serde_json::json;
use std::env;
use std::path::Path;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub(crate) struct DrainCli {
    #[structopt(flatten)]
    command: DrainCliCommand,
}

impl DrainCli {
    pub(crate) fn command(self) -> DrainCliCommand {
        self.command
    }
}

#[derive(StructOpt, Debug, Clone)]
pub(crate) struct DrainCliCommand {
    #[structopt(flatten)]
    output: Output,
    #[structopt(flatten)]
    selection: DrainSelection,
}

impl IntoIterator for DrainSelection {
    type Item = PathBuf;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let paths = match self {
            DrainSelection::All => vec![
                env::temp_dir().join("wasmcloudcache"),
                env::temp_dir().join("wasmcloud_ocicache"),
            ],
            DrainSelection::Oci => vec![env::temp_dir().join("wasmcloud_ocicache")],
            DrainSelection::Lib => vec![env::temp_dir().join("wasmcloudcache")],
        };
        paths.into_iter()
    }
}

#[derive(StructOpt, Debug, Clone)]
pub(crate) enum DrainSelection {
    /// TODO
    #[structopt(name = "all")]
    All,
    /// TODO
    #[structopt(name = "oci")]
    Oci,
    /// TODO
    #[structopt(name = "lib")]
    Lib,
}

pub(crate) fn handle_command(cmd: DrainCliCommand) -> Result<String, Box<dyn ::std::error::Error>> {
    let to_clear = cmd.selection.into_iter();
    let mut cleared = vec![];
    for path in to_clear {
        cleared.push(remove_dir_contents(path)?);
    }
    Ok(match cmd.output.kind {
        OutputKind::Text => format!("Successfully cleared caches at: {:?}", cleared),
        OutputKind::JSON => json!({ "drained": cleared }).to_string(),
    })
}

fn remove_dir_contents<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn ::std::error::Error>> {
    for entry in fs::read_dir(&path)? {
        let path = entry?.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else if path.is_file() {
            fs::remove_file(path)?;
        }
    }
    Ok(format!("{}", path.as_ref().display()))
}
