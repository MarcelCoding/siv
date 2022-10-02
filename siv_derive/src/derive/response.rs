use proc_macro2::TokenStream;
use quote::quote;
use syn::Data::Enum;
use syn::DeriveInput;

pub(crate) fn response(input: DeriveInput) -> TokenStream {
  let name = input.ident;
  let data = if let Enum(data) = input.data {
    data
  } else {
    panic!("Only supported on enums!")
  };

  let mut variants = Vec::with_capacity(data.variants.len());

  for variant in data.variants {
    let name = variant.ident;

    variants.push(quote! {
      Self::#name(data) => {
        let body = hyper::Body::from(serde_json::to_string(&data)?);
        hyper::Response::builder()
          .status(hyper::StatusCode::OK)
          .header(hyper::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
          .body(body)?
      }
    });
  }

  quote! {
    impl IntoResponse for #name {
      fn into_response(self) -> Result<hyper::Response<hyper::Body>, ResponseError> {
        let resp = match self {
          #(#variants),*
        };
        Result::Ok(resp)
      }
    }
  }
}
