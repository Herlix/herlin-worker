use proc_macro::TokenStream;
use syn;

#[proc_macro_derive(CloudFlareKV)]
pub fn documentize_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_cloudflare_kv(&ast)
}

fn impl_cloudflare_kv(ast: &syn::DeriveInput) -> TokenStream {
    r#"
       impl #name {
            pub async fn get(key: &str) -> wasm_bindgen::__rt::std::result::Result<Self, wasm_bindgen::__rt::std::io::Error> {
                let result = Self::call_js(js_get_#name(key)).await?.as_string().unwrap();
                let result = serde_json::from_str(result.as_str())?;
                Ok(result)
            }

            pub async fn put(key: &str, value: Self) -> wasm_bindgen::__rt::std::result::Result<Self, wasm_bindgen::__rt::std::io::Error> {
                let value = serde_json::to_string(&value)?;
                if let Ok(_) = Self::call_js(js_put_#name(key, value.as_str())).await {
                    Self::get(key).await
                } else {
                    Err(wasm_bindgen::__rt::std::io::Error::new(
                        wasm_bindgen::__rt::std::io::ErrorKind::NotFound,
                        format!("Could not find #name with id {}", key),
                    ))
                }
            }

            pub async fn delete(key: &str) -> wasm_bindgen::__rt::std::result::Result<bool, wasm_bindgen::__rt::std::io::Error> {
                match Self::call_js(js_delete_#name(key)).await {
                    Ok(_) => Ok(true),
                    Err(e) => Err(e),
                }
            }

            async fn call_js(promise: js_sys::Promise) -> wasm_bindgen::__rt::std::result::Result<wasm_bindgen::JsValue, wasm_bindgen::__rt::std::io::Error> {
                let result = wasm_bindgen_futures::JsFuture::from(promise).await.map_err(|e| {
                    wasm_bindgen::__rt::std::io::Error::new(
                        wasm_bindgen::__rt::std::io::ErrorKind::Other,
                        e.as_string().unwrap(),
                    )
                })?;
                Ok(result)
            }
        }

        #[wasm_bindgen::prelude::wasm_bindgen]
        extern "C" {
            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = get)]
            fn js_get_#name(key: &str) -> js_sys::Promise;

            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = put)]
            fn js_put_#name(key: &str, value: &str) -> js_sys::Promise;

            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = delete)]
            fn js_delete_#name(key: &str) -> js_sys::Promise;
        }"#
    .replace("#name", &ast.ident.to_string().as_str())
    .parse()
    .unwrap()
}
