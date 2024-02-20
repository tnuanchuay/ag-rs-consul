use std::fmt::Error;
use async_trait::async_trait;
use rs_consul::{Config, Consul, ReadKeyRequest, ReadKeyResponse};

#[async_trait]
pub trait KeyValueReader {
    async fn read_key(&self, key: &'static str) -> Option<String>;
}

struct AgConsul {
    dc: &'static str,
    consul_client: Consul
}

#[async_trait]
impl KeyValueReader for AgConsul {
    async fn read_key(&self, key: &'static str) -> Option<String>{
        let key_req = ReadKeyRequest {
            key,
            namespace: "",
            datacenter: self.dc,
            recurse: false,
            separator: "",
            consistency: Default::default(),
            index: None,
            wait: Default::default(),
        };

        let rkr = self.consul_client.read_key(key_req).await.unwrap_or_else(|err| {
            //TODO: @tnuanchuay please log error
            return Vec::new();
        });

        return Self::get_by_key(rkr, key);
    }
}

impl AgConsul {
    fn init_consul (url: &'static str) -> Consul {
        let config = Config {
            address: String::from(url),
            token: None,
            hyper_builder: Default::default(),
        };

        return Consul::new(config);
    }

    fn default(&self, url: &'static str, dc: &'static str) -> AgConsul{
        AgConsul {
            dc,
            consul_client: Self::init_consul(url),
        }
    }

    fn get_by_key<'a>(read_key_response: Vec<ReadKeyResponse>, key: &'static str) -> Option<String> {
        for key_res in read_key_response.iter() {
            if key_res.key == key {
                let r = key_res.clone();
                return r.value
            }
        }

        return None
    }
}

