use crate::ResponseError;

pub trait IntoResponse {
  fn into_response(self) -> Result<hyper::Response<hyper::Body>, ResponseError>;
}

#[cfg(test)]
mod test {
  use serde::Serialize;

  use siv_derive::Response;

  use crate::IntoResponse;
  use crate::ResponseError;

  #[derive(Serialize)]
  struct OkResponse {}

  #[derive(Serialize)]
  struct NotFoundResponse {}

  #[derive(Response)]
  enum HelloResponse {
    Ok(OkResponse),
    NotFound(NotFoundResponse),
  }
}
