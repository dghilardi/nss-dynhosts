use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::process::Command;
use anyhow::{bail, Context};
use handlebars::Handlebars;
use libnss::host::Addresses;
use serde::Serialize;
use crate::conf::CommandResolverConf;
use crate::resolver::CustomResolver;

#[derive(Serialize)]
pub struct TemplateParams {
    url: UrlParam,
}

#[derive(Serialize)]
pub struct UrlParam {
    full: String,
    parts: HashMap<usize, String>,
}

impl CustomResolver for CommandResolverConf {
    fn resolve(&self, name: &str) -> anyhow::Result<Option<Addresses>> {
        let h = Handlebars::new();
        let params = TemplateParams {
            url: UrlParam {
                full: String::from(name),
                parts: name.split('.')
                    .rev()
                    .enumerate()
                    .map(|(idx, s)| (idx, String::from(s)))
                    .collect(),
            },
        };

        let out = Command::new(&h.render_template(&self.command, &params)?)
            .args(&self.args.iter().map(|arg| h.render_template(arg, &params)).collect::<Result<Vec<_>, _>>()?)
            .current_dir(self.working_dir.clone().unwrap_or_else(|| PathBuf::from("/tmp")))
            .output()?;

        if out.status.success() {
            let out_str = String::from_utf8(out.stdout)?;
            if out_str.trim().is_empty() {
                Ok(None)
            } else {
                let ip: Ipv4Addr = out_str.trim().parse()
                    .with_context(|| format!("Error parsing '{out_str}'"))?;
                Ok(Some(Addresses::V4(vec![ip])))
            }
        } else {
            bail!("Error executing command - {}", out.status)
        }
    }
}