use ini::Ini;
use base64::prelude::*;

pub trait IntoWgConf: IntoWgConfSealed {}

impl IntoWgConf for WireGuardConfig {}

pub trait IntoWgConfSealed {
    fn into_wireguard_config(self) -> crate::Result<WireGuardConfig>;

}

impl IntoWgConfSealed for WireGuardConfig {
    fn into_wireguard_config(self) -> crate::Result<WireGuardConfig> {
        Ok(self)
    }

}

#[derive(Debug,Clone)]
pub(crate) struct WireGuardPeer {
    pub public_key: [u8; 32],
    pub allowed_ips: Vec<String>,
    pub endpoint: String,
    pub persistent_keepalive: Option<u16>,
}

#[derive(Debug,Clone)]
pub(crate) struct WireGuardConfig {
    pub private_key: [u8; 32],
    pub address: Vec<String>,
    pub listen_port: Option<u16>,
    pub peers: Vec<WireGuardPeer>,
}

impl WireGuardConfig {
    pub fn from_str(wireguard_config_str: &str) -> anyhow::Result<Self, anyhow::Error> {
        let conf = match Ini::load_from_str(wireguard_config_str) {
            Ok(conf) => conf,
            Err(_) => anyhow::bail!("failed to load wireguard config from string"),
        };
          
        // Parse Interface section
        let interface = match conf.section(Some("Interface")) {
            Some(interface) => interface,
            None => anyhow::bail!("Missing Interface section"),
        };
        
        let private_key = match interface.get("PrivateKey")
        {
            Some(private_key) => {
                let mut pk_bytes: [u8; 32] = [0u8; 32];
                BASE64_STANDARD.decode_slice(private_key,&mut pk_bytes).unwrap();
                pk_bytes
            },
            None => anyhow::bail!("Missing PrivateKey"),
        };
        
        let address = match interface.get("Address") {
            Some(addrs) => addrs.split(',')  .map(|s| s.trim().to_string()).collect(),
            None => anyhow::bail!("Missing Address"),
        };
            
        let listen_port = match interface.get("ListenPort")
            .map(|p| p.parse::<u16>())
            .transpose() {
                Ok(listen_port) => listen_port,
                Err(_) => None,
            };
        
        // Parse Peer sections
        let mut peers = Vec::new();
        for (section_name, section) in conf.iter() {
            if section_name.map_or(false, |name| name.starts_with("Peer")) {
                let peer = WireGuardPeer {
                    public_key: match section.get("PublicKey") {
                        Some(public_key) => {
                            let mut pk_slice: [u8; 32] = [0u8; 32];
                            BASE64_STANDARD.decode_slice(public_key, &mut pk_slice).unwrap();
                            pk_slice
                        },
                        None => anyhow::bail!("Missing PublicKey in Peer"),
                    },
                    allowed_ips: match section.get("AllowedIPs") {
                        Some(allowed_ips) => allowed_ips.split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>(),
                        None => anyhow::bail!("Missing AllowedIPs in Peer"),
                    },
                    endpoint: match section.get("Endpoint").map(String::from) {
                        Some(endpoint) => endpoint,
                        None => anyhow::bail!("Missing Endpoint in Peer"),
                    },
                    persistent_keepalive: section.get("PersistentKeepalive")
                        .map(|v| v.parse::<u16>())
                        .transpose()?,
                };
                peers.push(peer);
            }
        }
        
        Ok(WireGuardConfig {
            private_key,
            address,
            listen_port,
            peers,
        })
    }
}
