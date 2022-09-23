mod bin;
mod conn;
mod r#macro;
mod r#trait;

use std::collections::HashMap;

use fred::{
  interfaces::{FunctionInterface, HashesInterface, KeysInterface, SetsInterface},
  prelude::{AsyncResult, RedisClient},
  types::{Expiration, FromRedis},
};
use napi::bindgen_prelude::Uint8Array;
use napi_derive::napi;

use crate::{
  bin::{Bin, Bins},
  r#trait::To,
};

#[napi]
pub struct Redis(RedisClient);

fn fcall_ro<T: Send + Unpin + FromRedis + 'static>(
  redis: &Redis,
  name: Bin,
  keys: Option<Vec<Bin>>,
  vals: Option<Vec<Bin>>,
) -> AsyncResult<T> {
  redis.0.fcall_ro::<T, _, _, _>(name, keys, vals)
}

fn fcall<T: Send + Unpin + FromRedis + 'static>(
  redis: &Redis,
  name: Bin,
  keys: Option<Vec<Bin>>,
  vals: Option<Vec<Bin>>,
) -> AsyncResult<T> {
  match keys {
    Some(keys) => redis.0.fcall::<T, _, _, _>(name, keys, vals),
    None => fcall_ro::<T>(redis, name, None, vals),
  }
}

napiImpl!(Redis :

get(&self, key:Bin) -> Option<String> {
  self.0.get::<Option<String>, _>(key).await?
}

get_b(&self, key:Bin) -> Option<Uint8Array> {
  self.0.get::<Option<Vec<u8>>, _>(key).await?
}

set(&self, key:Bin, val:Bin) -> (){
  self.0.set::<(),_,_>(
    key,
    val,
    None,
    None,
    false
  ).await?
}

setex(&self, key:Bin, val:Bin, seconds:i64) -> (){
  self.0.set::<(),_,_>(
    key,
    val,
    Some(Expiration::EX(seconds)),
    None,
    false
  ).await?
}

expire(&self, key:Bin, seconds:i64) -> bool {
  self.0.expire::<bool,_>(
    key,
    seconds
  ).await?
}

del(&self, key:Bins) -> u32 {
  self.0.del::<u32,_>(key).await?
}

exist(&self, key:Bins) -> u32 {
  self.0.exists::<u32,_>(key).await?
}

hget(&self, key:Bin, field:Bin) -> Option<String> {
  self.0.hget::<Option<String>,_,_>(key, field).await?
}

hget_b(&self, key:Bin, field:Bin) -> Option<Uint8Array> {
  self.0.hget::<Option<Vec<u8>>,_,_>(key, field).await?
}

hget_i(&self, key:Bin, field:Bin) -> Option<i64> {
  self.0.hget::<Option<i64>,_,_>(key, field).await?
}

hset(&self, key:Bin, map:HashMap::<String,Bin>) -> () {
  self.0.hset::<(),_,_>(key, map).await?
}

hincrby(&self, key:Bin, field:Bin, increment:i64) -> i64 {
  self.0.hincrby::<i64,_,_>(key, field, increment).await?
}

hincr(&self, key:Bin, field:Bin) -> i64 {
  self.0.hincrby::<i64,_,_>(key, field, 1).await?
}

hexists(&self, key:Bin, field:Bin) -> bool {
  self.0.hexists::<bool,_,_>(key, field).await?
}

sadd(&self, key:Bin, members:Bins) -> u32 {
  self.0.sadd::<u32,_,_>(key, members).await?
}

fnload(&self, script:Bin) -> String {
  self.0.function_load::<String,_>(true,script).await?
}

fbin(&self, name:Bin, keys:Option<Vec<Bin>>, vals:Option<Vec<Bin>>) -> Uint8Array {
  fcall::<Vec<u8>>(self,name,keys,vals).await?
}

fstr(&self, name:Bin, keys:Option<Vec<Bin>>, vals:Option<Vec<Bin>>) -> String {
  fcall::<String>(self,name,keys,vals).await?
}

fi64(&self, name:Bin, keys:Option<Vec<Bin>>, vals:Option<Vec<Bin>>) -> i64 {
  fcall::<i64>(self,name,keys,vals).await?
}

fcall(&self, name:Bin, keys:Option<Vec<Bin>>, vals:Option<Vec<Bin>>) -> () {
  fcall::<()>(self,name,keys,vals).await?
}

fbool_ro(&self, name:Bin, keys:Option<Vec<Bin>>, vals:Option<Vec<Bin>>) -> bool {
  fcall_ro::<bool>(self, name, keys, vals).await?
}

);
