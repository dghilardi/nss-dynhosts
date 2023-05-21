use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::process::Command;
use anyhow::bail;
use libnss::host::Addresses;
use crate::conf::CommandResolverConf;
use crate::resolver::CustomResolver;

impl CustomResolver for CommandResolverConf {
    fn resolve(&self, name: &str) -> anyhow::Result<Option<Addresses>> {
        let out = Command::new(&self.command)
            .args(&self.args)
            .current_dir(self.working_dir.clone().unwrap_or_else(|| PathBuf::from("/tmp")))
            .output()?;

        if out.status.success() {
            let out_str = String::from_utf8(out.stdout)?;
            if out_str.trim().is_empty() {
                Ok(None)
            } else {
                let ip: Ipv4Addr = out_str.parse()?;
                Ok(Some(Addresses::V4(vec![ip])))
            }
        } else {
            bail!("Error executing command - {}", out.status)
        }
    }
}