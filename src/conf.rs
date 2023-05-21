use std::fs;
use std::path::{Path, PathBuf};
use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct DynHostsConf {
    pub hosts: IndexMap<String, ResolverConf>
}

#[derive(Deserialize)]
pub enum ResolverConf {
    Command(CommandResolverConf),
}

#[derive(Deserialize)]
pub struct CommandResolverConf {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<PathBuf>,
}

pub fn read_conf() -> DynHostsConf {
    let conf_path = PathBuf::from("/etc/dynhosts.toml");
    if !conf_path.is_file() {
        return DynHostsConf::default();
    }

    let conf_res = parse_conf(&conf_path);

    match conf_res {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Error reading conf - {err}");
            DynHostsConf::default()
        }
    }
}

fn parse_conf(path: &Path) -> anyhow::Result<DynHostsConf> {
    let conf_str = fs::read_to_string(path)?;
    Ok(toml::from_str(&conf_str)?)
}