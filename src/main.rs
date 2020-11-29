#![warn(unused_variables)]

#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! { }
}

//--------------------------------------------------------------------------------------

#[macro_use]
extern crate gtmpl_derive;

//extern crate gtmpl_value;
//use gtmpl_value;

//--------------------------------------------------------------------------------------

use std::fs;
use std::time;
use std::time::Duration;
use std::path::PathBuf;
use std::convert::TryInto;
use uuid::Uuid;
use uuid::v1::Timestamp;
use std::vec::Vec;


fn now() -> Result<Duration, Box<dyn std::error::Error + 'static>> {
    Ok(time::SystemTime::now().duration_since(time::UNIX_EPOCH)?)
}
    
fn timestamp() -> Result<Timestamp, Box<dyn std::error::Error + 'static>> {
    use std::process;
    use uuid::v1::Context;
    let context = Context::new((process::id() / 0xff).try_into()?);
    let systime = now()?;
    Ok(Timestamp::from_unix(&context, systime.as_secs(), (systime.as_nanos() / 0xffff).try_into()?))
}

fn macaddress(prefix: u16, cluster: u16, node: u16) -> Result<String, Box<dyn std::error::Error + 'static>> {
    if prefix & 0x0200_u16 == 0 {
        error_chain::bail!(format!("Prefix cannot be accepted for a MAC LAA: {:04x}", prefix))
    } else {
        Ok(format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                   (prefix  & 0xff00_u16) >> 8, prefix  & 0x00ff_u16,
                   (cluster & 0xff00_u16) >> 8, cluster & 0x00ff_u16,
                   (node    & 0xff00_u16) >> 8, node    & 0x00ff_u16))
    }
}

fn config(cluster: u16, gpaths: &GlobalPaths, rpaths: &RelativePaths, params: &Params)
          -> Result<Config, Box<dyn std::error::Error + 'static>> {
    
    use errors::*;
    
    let tstamp = timestamp()?;

    // global parameters
    let globals = Globals {
        src:      gpaths.src.clone(),
        dst:      gpaths.dst.clone(),
        boot:     gpaths.boot.clone(),
        pools:    rpaths.pools.clone(),
        networks: rpaths.networks.clone(),
        volumes:  rpaths.volumes.clone(),
        nodes:    rpaths.nodes.clone(),
    };

    // network definitiions
    let network = Network {
        name: params.network.clone(),
        uuid: Uuid::new_v5(&Uuid::NAMESPACE_DNS, params.network.as_bytes()).to_string(),
        domain: format!("{:04x}.{}", params.cluster, params.domain),
        cluster: format!("{:04x}", params.cluster),
        intf: params.network_intf.clone(),
        bridge: params.network_bridge.clone(),
        macaddress: macaddress(0x0200, cluster, 0)?
    };

    // pool definitions
    let images = Pool {
        name: "images".to_string(),
        uuid: Uuid::new_v1(tstamp.clone(), "images".as_bytes())
            .chain_err(|| "failed to generate UUID for pool: images")?
            .to_string(),
    };
    let combustion = Pool {
        name: "combustion".to_string(),
        uuid: Uuid::new_v1(tstamp.clone(), "combustion".as_bytes())
            .chain_err(|| "failed to generate UUID for pool: combustion")?
            .to_string(),
    };
    let pools = vec!(images, combustion);

    let result = Config {
        globals,
        network,
        pools,
        nodes: vec!(),
    };
    
    // volume definitions
    //++ let rcs = (1..params.masters).map(|n| Combustion {
    //++     hostname: format!("master{}", n),
    //++     domain: format!("{:04x}.{}", params.cluster, params.domain),
    //++     packages: "fd ripgrep tree htop traceroute zile".to_string(),
    //++     rootpw: params.rootpw,
    //++ }).collect();

    // node definitions
    //++ let masters = (1..params.masters).map(|n| Node {
    //++     name: format!("master{}", n),
    //++     uuid: Uuid::new_v5(&Uuid::NAMESPACE_DNS, format!("master{}.{:04x}.{}", n, params.cluster, params.domain).as_bytes()),
    //++     cluster: format!("{:04x}", params.cluster),
    //++     volsys: format!("/var/lib/libvirt/images/{:04x}-master{}-system.qcow2", params.cluster, n),
    //++     volrc: format!("{}/{}/{:04x}-master{}-combustion.iso", globals.dst, globals.volumes, params.cluster, n),
    //++     network: params.network,
    //++     macaddress: format!("{}:{}:{}:{}",
    //++                         params.network_macaddress_prefix,
    //++                         format!("{:02x}", params.cluster / 0xff),
    //++                         format!("{:02x}", params.cluster % 0xff),
    //++                         format!("{:02x}", n)),
    //++ }).collect();

    Ok(result)
}


fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let gpaths = GlobalPaths {
        src:  "/mnt/server/2min-virsh-template".to_string(),
        dst:  "/mnt/server/2min-virsh".to_string(),
        boot: "/mnt/server/Software/images".to_string(),
    };

    let rpaths = RelativePaths {
        pools:    "/config/pools".to_string(),
        networks: "/vm-manager/networks".to_string(),
        volumes:  "/vm-manager/volumes".to_string(),
        nodes:    "/vm-manager/nodes".to_string(),
    };

    let params = Params {
        cluster: 0xf900,
        domain: "2mincloud.com".to_string(),
        masters: 3,
        workers: 4,
        rootpw: "$6$ZuqvUx9NJXCrOlxg$nZ7vaEktmlR3qLtnau/5yuC46AHVQLV.mBm3d3a49zC1GoY1Krr5/4wfFTYrYJh.eGDJkkTScT8/kOxwF0kwu.".to_string(),
        network: "2mincloud".to_string(),
        network_intf: "enp5s0".to_string(),
        network_bridge: "virbr1".to_string(),
        network_macaddress_prefix: "52:54:00".to_string(),
    };
    
    let c = config(0xf900, &gpaths, &rpaths, &params);

    // generate files
    //TODO:: create_network(&globals, &network)
    Ok(())
}

fn create_network(globals: &Globals, network: &Network) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let is = PathBuf::from(format!("{}/{}/network.xml", globals.src, globals.networks));
    let os = PathBuf::from(format!("{}/{}/{}.xml",      globals.dst, globals.networks, network.name));
    let text = gtmpl::template(&fs::read_to_string(is)?, network.clone());
    let dir = os.parent().expect("Could not obtain dirname");
    fs::create_dir_all(dir)?;
    Ok(fs::write(os, text?)?)
}

//--------------------------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq)]
struct GlobalPaths {
    src: String,
    dst: String,
    boot: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RelativePaths {
    pools: String,
    networks: String,
    volumes: String,
    nodes: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Params {
    cluster: u32,
    domain: String,
    masters: u16,
    workers: u16,
    rootpw: String,
    network: String,
    network_intf: String,
    network_bridge: String,
    network_macaddress_prefix: String,
}

//--------------------------------------------------------------------------------------

struct Config {
    globals: Globals,
    network: Network,
    pools: Vec<Pool>,
    nodes: Vec<Node>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Globals {
    src: String,
    dst: String,
    boot: String,
    pools: String,
    networks: String,
    volumes: String,
    nodes: String,
}

#[derive(Gtmpl)]
#[derive(Debug, Clone, Eq, PartialEq)]
struct Network {
    name: String,
    uuid: String,
    domain: String,
    cluster: String,
    intf: String,
    bridge: String,
    macaddress: String,
}

#[derive(Gtmpl)]
#[derive(Debug, Clone, Eq, PartialEq)]
struct Pool {
    name: String,
    uuid: String,
}

#[derive(Gtmpl)]
#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    name: String,
    uuid: String,
    cluster: String,
    volsys: String,
    volrc: String,
    network: String,
    macaddress: String,
    combustion: Combustion,
}

#[derive(Gtmpl)]
#[derive(Debug, Clone, Eq, PartialEq)]
struct Combustion {
    hostname: String,
    domain: String,
    packages: String,
    rootpw: String,
}



#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use uuid::Uuid;
    use crate::GlobalPaths;
    use crate::Params;
    use crate::RelativePaths;

    use super::macaddress;
    use super::config;

    #[test]
    #[should_panic]
    fn macaddress_is_valid() {
        let mac = macaddress(0x0100, 0xcafe, 0xbabe).expect("could not obtain mac address");
        assert_eq!("02:00:ca:fe:ba:be", mac);
    }
        
    #[test]
    fn macaddress_is_rejected()  -> Result<(), Box<dyn std::error::Error + 'static>> {
        let mac = macaddress(0x0200, 0xcafe, 0xbabe)?;
        assert_eq!("02:00:ca:fe:ba:be", mac);
        Ok(())
    }

    #[test]
    fn configuration_is_valid() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let gpaths = GlobalPaths {
            src:  "/mnt/server/2min-virsh-template".to_string(),
            dst:  "/mnt/server/2min-virsh".to_string(),
            boot: "/mnt/server/Software/images".to_string(),
        };
     
        let rpaths = RelativePaths {
            pools:    "/config/pools".to_string(),
            networks: "/vm-manager/networks".to_string(),
            volumes:  "/vm-manager/volumes".to_string(),
            nodes:    "/vm-manager/nodes".to_string(),
        };
     
        let params = Params {
            cluster: 0xf900,
            domain: "2mincloud.com".to_string(),
            masters: 3,
            workers: 4,
            rootpw: "$6$ZuqvUx9NJXCrOlxg$nZ7vaEktmlR3qLtnau/5yuC46AHVQLV.mBm3d3a49zC1GoY1Krr5/4wfFTYrYJh.eGDJkkTScT8/kOxwF0kwu.".to_string(),
            network: "2mincloud".to_string(),
            network_intf: "enp5s0".to_string(),
            network_bridge: "virbr1".to_string(),
            network_macaddress_prefix: "52:54:00".to_string(),
        };
        
        let globals = super::Globals {
            src:    "/mnt/server/2min-virsh-template".to_string(),
            dst:    "/mnt/server/2min-virsh".to_string(),
            boot:   "/mnt/server/Software/images".to_string(),
            pools:    "/config/pools".to_string(),
            networks: "/vm-manager/networks".to_string(),
            volumes:  "/vm-manager/volumes".to_string(),
            nodes:    "/vm-manager/nodes".to_string(),
        };

        let network = super::Network {
            name: "2mincloud".to_string(),
            domain: "2mincloud.com".to_string(),
            cluster: "f900".to_string(),
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_DNS, "2mincloud".as_bytes()).to_string(),
            intf: "enp5s0".to_string(),
            bridge: "virbr1".to_string(),
            macaddress: "52:54:00:f9:01:00".to_string(),
        };

        let c = config(0xf900, &gpaths, &rpaths, &params)?;
        assert_eq!(c.globals, globals);
        assert_eq!(c.network, network);
        assert_eq!(c.pools.get(0).unwrap().name, "images");
        assert_eq!(c.pools.get(1).unwrap().name, "combustion");

        Ok(())
    }
    
    fn render_globals() -> Result<(), Box<dyn std::error::Error + 'static>> {
        let gpaths = GlobalPaths {
            src:  "/mnt/server/2min-virsh-template".to_string(),
            dst:  "/mnt/server/2min-virsh".to_string(),
            boot: "/mnt/server/Software/images".to_string(),
        };
     
        let rpaths = RelativePaths {
            pools:    "/config/pools".to_string(),
            networks: "/vm-manager/networks".to_string(),
            volumes:  "/vm-manager/volumes".to_string(),
            nodes:    "/vm-manager/nodes".to_string(),
        };
     
        let params = Params {
            cluster: 0xf900,
            domain: "2mincloud.com".to_string(),
            masters: 3,
            workers: 4,
            rootpw: "$6$ZuqvUx9NJXCrOlxg$nZ7vaEktmlR3qLtnau/5yuC46AHVQLV.mBm3d3a49zC1GoY1Krr5/4wfFTYrYJh.eGDJkkTScT8/kOxwF0kwu.".to_string(),
            network: "2mincloud".to_string(),
            network_intf: "enp5s0".to_string(),
            network_bridge: "virbr1".to_string(),
            network_macaddress_prefix: "52:54:00".to_string(),
        };

        let globals = super::Globals {
            src:    "/mnt/server/2min-virsh-template".to_string(),
            dst:    "/mnt/server/2min-virsh".to_string(),
            boot:   "/mnt/server/Software/images".to_string(),
            pools:    "/config/pools".to_string(),
            networks: "/vm-manager/networks".to_string(),
            volumes:  "/vm-manager/volumes".to_string(),
            nodes:    "/vm-manager/nodes".to_string(),
        };

        let network = super::Network {
            name: "2mincloud".to_string(),
            domain: "2mincloud.com".to_string(),
            cluster: "f900".to_string(),
            uuid: Uuid::new_v5(&Uuid::NAMESPACE_DNS, "2mincloud".as_bytes()).to_string(),
            intf: "enp5s0".to_string(),
            bridge: "virbr1".to_string(),
            macaddress: "52:54:00:f9:01:00".to_string(),
        };

        let is = PathBuf::from(format!("{}/{}/network.xml", globals.src, globals.networks));
        let os = PathBuf::from(format!("{}/{}/{}.xml",      globals.dst, globals.networks, network.name));
        assert_eq!(is, PathBuf::from("/mnt/server/2min-virsh-template/vm-manager/networks/network.xml"));
        assert_eq!(os, PathBuf::from("/mnt/server/2min-virsh-tests/vm-manager/networks/2mincloud.xml"));
        let text = gtmpl::template(&fs::read_to_string(is)?, network.clone());
        let dir = os.parent().expect("Could not obtain dirname");
        assert_eq!(dir, PathBuf::from("/mnt/server/2min-virsh-tests/vm-manager/networks"));
        fs::create_dir_all(dir)?;
        Ok(fs::write(os, text?)?)
    }
}
