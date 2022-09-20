use fred::{
  bytes_utils::Str,
  types::{RedisKey, RedisValue},
};
use napi::{
  bindgen_prelude::{Buffer, FromNapiValue},
  sys::{napi_env, napi_value},
  Either,
};
pub type StringOrBuffer = Either<String, Buffer>;

pub struct Bin(StringOrBuffer);

impl AsRef<[u8]> for Bin {
  fn as_ref(&self) -> &[u8] {
    self.0.as_ref()
  }
}

impl FromNapiValue for Bin {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(Bin(StringOrBuffer::from_napi_value(env, napi_val)?))
  }
}

impl From<Bin> for RedisKey {
  fn from(t: Bin) -> RedisKey {
    RedisKey::from(t.0.as_ref())
  }
}

impl From<Bin> for Str {
  fn from(t: Bin) -> Str {
    String::from_utf8_lossy(t.0.as_ref()).into()
  }
}

impl From<Bin> for RedisValue {
  fn from(t: Bin) -> RedisValue {
    RedisValue::from(t.0.as_ref())
  }
}
