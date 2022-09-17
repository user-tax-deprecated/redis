mod bin;
mod r#macro;

use fred::{
  interfaces::{ClientLike, KeysInterface},
  prelude::{ReconnectPolicy, RedisClient, RedisConfig, ServerConfig},
};
use napi::bindgen_prelude::{External, Uint8Array};
use napi_derive::napi;

use crate::bin::Bin;

#[napi]
pub struct Redis(RedisClient);

napiImpl!(

Redis :

  get(&self, key:Bin) -> Option<String> {
    self.0.get::<Option<String>, _>(key).await
  }

  get_b(&self, key:Bin) -> Option<Uint8Array> {
    OptionInto!({
      self.0.get::<Option<Vec<u8>>, _>(key)
    })
  }

  set(&self, key:Bin, val:Bin) -> (){
    self.0.set::<(),_,_>(
      key,
      val,
      None,
      None,
      false
    ).await
  }

  del(&self, key:Bin) -> u32 {
    self.0.del::<u32,_>(key).await
  }

  mdel(&self, key_li:Vec<Bin>) -> u32 {
    self.0.del::<u32,_>(key_li).await
  }

);

#[napi]
pub fn server_cluster(hosts: Vec<(String, u16)>) -> External<ServerConfig> {
  ServerConfig::Clustered { hosts }.into()
}

#[napi]
pub fn server_host_port(host: String, port: u16) -> External<ServerConfig> {
  ServerConfig::Centralized { host, port }.into()
}

def!(redis_conn(
    version: u8,
    server:External<ServerConfig>,
    username:Option<String>,
    password:Option<String>,
    db:Option<u8>
) -> Redis {
  let server = server.as_ref().clone();
  let mut config = RedisConfig::default();
  if version == 3 {
    config.version = fred::types::RespVersion::RESP3;
  }
  config.server = server;
  config.database = db;
  config.password = password;
  config.username = username;
  // configure exponential backoff when reconnecting, starting at 100 ms, and doubling each time up to 30 sec.
  let policy = ReconnectPolicy::new_exponential(0, 100, 30_000, 2);

  let client = RedisClient::new(config);
  let _ = client.connect(Some(policy));
  client.wait_for_connect().await?;

  Redis( client )
});
