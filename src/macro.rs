#[macro_export]
macro_rules! def {
  (
    $fn:ident ( $($name:ident : $type:ty ),*) -> $result:ty $body:block
  )=>{
#[napi]
    pub async fn $fn($($name : $type),*) -> napi::Result<$result> {
      Ok({
        async move {
          Ok($body)
        }
        .await as anyhow::Result<_, anyhow::Error>
      }?)
    }
  };
}

#[macro_export]
macro_rules! OptionInto {
  ($body:block) => {
    Ok::<_, anyhow::Error>(match $body.await? {
      Some(r) => Some(r.into()),
      _ => None,
    })
  };
}

#[macro_export]
macro_rules! napiImpl {
  (
    $cls:ty :
    $(
      $fn:ident (&$($name:ident $(: $type:ty)? ),*) -> $result:ty $body:block
    )*
  )=>{
#[napi]
    impl $cls {
      $(
        #[napi]
        pub async fn $fn(&$($name$(: $type )?,)*) -> napi::Result<$result> {
          Ok({
            async move {
              Ok($body?.into())
            }
            .await as anyhow::Result<_, anyhow::Error>
          }?)
        }
      )*
    }
  }
}
