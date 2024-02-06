use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn actually_do_the_macro(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = input.ident;
    Ok(quote! {
        impl axum::response::IntoResponse for #ident {
            fn into_response(self) -> axum::http::Response<axum::body::Body> {
                let mut res = axum::body::Body::new(serde_json::to_string(&self).unwrap()).into_response();
                res.headers_mut().insert("content-type", unsafe { axum::http::HeaderValue::from_str("application/json").unwrap_unchecked() });
                res
            }
        }
    })
}

#[proc_macro_derive(Response)]
pub fn response_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    actually_do_the_macro(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
