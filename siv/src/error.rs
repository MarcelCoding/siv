macro_rules! from_error {
  ($vis:vis enum $custom:ident {$($name:ident($error:ty)),+}) => {
    $vis enum $custom {
      $($name($error)),+
    }

    $(impl From<$error> for $custom {
      fn from(err: $error) -> Self {
        Self::$name(err)
      }
    })+

    impl std::fmt::Debug for $custom {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $(Self::$name(err) => {
            f.write_str(concat!(stringify!($name), " Error: "))?;
            std::fmt::Debug::fmt(err, f)
          }),+
        }
      }
    }

    impl std::fmt::Display for $custom {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
          $(Self::$name(err) => {
            f.write_str(concat!(stringify!($name), " Error: "))?;
            std::fmt::Display::fmt(err, f)
          }),+
        }
      }
    }
  }
}

from_error! {
  pub enum ResponseError {
    Http(hyper::http::Error),
    Json(serde_json::Error)
  }
}
