use napi::bindgen_prelude::Uint8Array;

pub trait To<T> {
  fn to(self) -> T;
}

impl<X> To<X> for X {
  fn to(self) -> X {
    self
  }
}

impl To<Option<Uint8Array>> for Option<Vec<u8>> {
  fn to(self) -> Option<Uint8Array> {
    match self {
      Some(x) => Some(x.into()),
      None => None,
    }
  }
}
