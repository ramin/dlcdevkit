use bitcoin::{secp256k1::Secp256k1, KeyPair, XOnlyPublicKey};
use std::{marker::PhantomData, str::FromStr};

const ORACLE_URL: &str = "http://localhost:8080";

fn get<T>(path: &str) -> anyhow::Result<T> where T: serde::de::DeserializeOwned {
    let url = format!("{}{}", ORACLE_URL, path);
    let request = reqwest::blocking::get(url)?.json::<T>()?;
    
    Ok(request)
}

#[derive(Debug)]
pub struct ErnestOracle {
    pubkey: XOnlyPublicKey
}

impl ErnestOracle {
    pub fn new() -> anyhow::Result<ErnestOracle> {
        let request: String = get("/pubkey")?;
        let pubkey = XOnlyPublicKey::from_str(&request)?; 
        Ok(ErnestOracle { pubkey })
    }

    pub fn get_pubkey(&self) -> anyhow::Result<XOnlyPublicKey> {
        let request: String = get("/pubkey")?;
        Ok(XOnlyPublicKey::from_str(&request)?)
    }

}

impl dlc_manager::Oracle for ErnestOracle {
    fn get_public_key(&self) -> bitcoin::XOnlyPublicKey {
        self.pubkey
    }

    fn get_attestation(
        &self,
        _event_id: &str,
    ) -> Result<dlc_messages::oracle_msgs::OracleAttestation, dlc_manager::error::Error> {
        unimplemented!("attestation")
    }

    fn get_announcement(
        &self,
        _event_id: &str,
    ) -> Result<dlc_messages::oracle_msgs::OracleAnnouncement, dlc_manager::error::Error> {
        unimplemented!("announcement")
    }
}
