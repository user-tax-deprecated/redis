pub trait To<T> {
  fn to(self) -> T;
}

impl<X, T: Into<X>> To<Option<X>> for Option<T> {
  fn to(self) -> Option<X> {
    match self {
      Some(x) => Some(x.into()),
      None => None,
    }
  }
}

impl<X, T: Into<X>> To<X> for T {
  fn to(self) -> X {
    self.to()
  }
}
