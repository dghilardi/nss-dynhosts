mod command;

use std::net::IpAddr;
use libnss::host::Addresses;
use crate::conf::ResolverConf;

pub trait CustomResolver {
    fn resolve(&self, name: &str) -> anyhow::Result<Option<Addresses>>;
}

impl CustomResolver for ResolverConf {
    fn resolve(&self, name: &str) -> anyhow::Result<Option<Addresses>> {
        match self {
            ResolverConf::Command(cmd_conf) => cmd_conf.resolve(name),
        }
    }
}