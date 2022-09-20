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
    self.map(|x| x.into())
  }
}

impl To<Uint8Array> for Vec<u8> {
  fn to(self) -> Uint8Array {
    self.into()
  }
}
