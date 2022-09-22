use std::iter::IntoIterator;

use fred::{
  bytes_utils::Str,
  types::{MultipleKeys, MultipleValues, RedisKey, RedisValue},
};
use napi::{
  bindgen_prelude::{Buffer, FromNapiValue, TypeName, ValidateNapiValue, ValueType},
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

impl IntoIterator for Bin {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = RedisKey;

  fn into_iter(self) -> Self::IntoIter {
    vec![self.into()].into_iter()
  }
}

impl TypeName for Bin {
  fn type_name() -> &'static str {
    "String/Buffer/Uint8Array"
  }

  fn value_type() -> napi::ValueType {
    StringOrBuffer::value_type()
  }
}

impl ValidateNapiValue for Bin {
  unsafe fn validate(env: napi_env, napi_val: napi_value) -> napi::Result<napi_value> {
    unsafe { StringOrBuffer::validate(env, napi_val) }
  }
}

impl FromNapiValue for Bin {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(Self(StringOrBuffer::from_napi_value(env, napi_val)?))
  }
}

impl From<Bin> for RedisKey {
  fn from(t: Bin) -> RedisKey {
    RedisKey::from(t.0.as_ref())
  }
}

impl From<Bin> for RedisValue {
  fn from(t: Bin) -> RedisValue {
    RedisValue::from(t.0.as_ref())
  }
}

impl From<Bin> for Str {
  fn from(t: Bin) -> Str {
    String::from_utf8_lossy(t.0.as_ref()).into()
  }
}

pub type EitherBinVec = Either<Bin, Vec<Bin>>;

pub struct BinMaybeVec(EitherBinVec);

impl FromNapiValue for BinMaybeVec {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(Self(EitherBinVec::from_napi_value(env, napi_val)?))
  }
}

impl From<BinMaybeVec> for MultipleKeys {
  fn from(t: BinMaybeVec) -> Self {
    match t.0 {
      EitherBinVec::A(a) => a.into(),
      EitherBinVec::B(b) => b.into(),
    }
  }
}

impl From<BinMaybeVec> for MultipleValues {
  fn from(t: BinMaybeVec) -> Self {
    match t.0 {
      EitherBinVec::A(a) => a.into(),
      EitherBinVec::B(b) => b.into_iter().into(),
    }
  }
}
