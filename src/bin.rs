use std::iter::IntoIterator;

use fred::{
  bytes_utils::Str,
  types::{MultipleKeys, MultipleValues, RedisKey, RedisValue},
};
use napi::{
  bindgen_prelude::{Buffer, Either3, FromNapiValue, TypeName, ValidateNapiValue},
  sys::{napi_env, napi_value},
  Either,
};

pub type StringOrBuffer = Either3<String, Buffer, i64>;

pub struct Bin(StringOrBuffer);

impl Bin {
  fn to_box(&self) -> Box<[u8]> {
    match &self.0 {
      StringOrBuffer::A(i) => AsRef::<[u8]>::as_ref(i).into(),
      StringOrBuffer::B(i) => AsRef::<[u8]>::as_ref(i).into(),
      StringOrBuffer::C(i) => i.to_string().as_bytes().into(),
    }
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
    RedisKey::from(&t.to_box()[..])
  }
}

impl From<Bin> for RedisValue {
  fn from(t: Bin) -> RedisValue {
    RedisValue::from(&t.to_box()[..])
  }
}

impl From<Bin> for Str {
  fn from(t: Bin) -> Str {
    String::from_utf8_lossy(&t.to_box()).into()
  }
}

pub type EitherBinVec = Either<Bin, Vec<Bin>>;

pub struct Bins(EitherBinVec);

impl FromNapiValue for Bins {
  unsafe fn from_napi_value(env: napi_env, napi_val: napi_value) -> napi::Result<Self> {
    Ok(Self(EitherBinVec::from_napi_value(env, napi_val)?))
  }
}

impl From<Bins> for MultipleKeys {
  fn from(t: Bins) -> Self {
    match t.0 {
      EitherBinVec::A(a) => a.into(),
      EitherBinVec::B(b) => b.into(),
    }
  }
}

impl From<Bins> for MultipleValues {
  fn from(t: Bins) -> Self {
    match t.0 {
      EitherBinVec::A(a) => a.into(),
      EitherBinVec::B(b) => b.into_iter().into(),
    }
  }
}
