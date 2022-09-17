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
