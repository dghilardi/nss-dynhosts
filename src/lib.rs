mod conf;
mod resolver;

use std::fs::File;
use std::net::{IpAddr, Ipv4Addr};
use libnss::host::{Addresses, AddressFamily, Host, HostHooks};
use libnss::interop::Response;
use libnss::libnss_host_hooks;
use crate::conf::{DynHostsConf, read_conf, ResolverConf};
use crate::resolver::CustomResolver;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONF: DynHostsConf = read_conf();
}

pub struct HostsPlumber;
libnss_host_hooks!(plumber, HostsPlumber);

impl HostHooks for HostsPlumber {
    fn get_all_entries() -> Response<Vec<Host>> {
        Response::Success(Vec::new())
    }

    fn get_host_by_name(name: &str, family: AddressFamily) -> Response<Host> {
        match family {
            AddressFamily::IPv4 => {
                let Some((_, resolver_conf)): Option<(&String, &ResolverConf)> = CONF.hosts.iter()
                    .filter(|(hostname, _)| name.ends_with(*hostname))
                    .next() else {
                    return Response::NotFound;
                };

                let resolve_result = resolver_conf.resolve(name);
                match resolve_result {
                    Ok(Some(addresses)) => Response::Success(Host {
                        name: String::from(name),
                        aliases: vec![],
                        addresses,
                    }),
                    Ok(None) => Response::NotFound,
                    Err(_err) => Response::Unavail,
                }
            },
            AddressFamily::IPv6 => Response::NotFound,
            AddressFamily::Unspecified => Response::NotFound,
        }
    }

    fn get_host_by_addr(addr: IpAddr) -> Response<Host> {
        Response::NotFound
    }
}