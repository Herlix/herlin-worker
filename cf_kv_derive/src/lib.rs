use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Ident;

#[proc_macro_derive(CloudFlareKV)]
pub fn cloudflare_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_cloudflare_kv(&ast)
}

fn impl_cloudflare_kv(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let js_get = Ident::new(&format!("js_get_{}", &name), ast.ident.span());
    let js_put = Ident::new(&format!("js_put_{}", &name), ast.ident.span());
    let js_delete = Ident::new(&format!("js_delete_{}", &name), ast.ident.span());

    let res = quote! {

        #[wasm_bindgen::prelude::wasm_bindgen]
        extern "C" {
            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = get)]
            fn #js_get(key: &str) -> js_sys::Promise;

            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = put)]
            fn #js_put(key: &str, value: &str) -> js_sys::Promise;

            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = delete)]
            fn #js_delete(key: &str) -> js_sys::Promise;
        }

        #[async_trait::async_trait(?Send)]
        impl cf_kv::CloudFlareKV<'_, #name> for #name {
            async fn get(
                key: &str,
            ) -> Result<wasm_bindgen::__rt::std::option::Option<#name>, wasm_bindgen::__rt::std::io::Error>
            {
                let result = Self::call_js(unsafe { #js_get(key) }).await?;
                if result.is_null() {
                    return Ok(wasm_bindgen::__rt::std::option::Option::None);
                }
                let user_info = serde_json::from_str(result.as_string().unwrap().as_str())?;
                Ok(wasm_bindgen::__rt::std::option::Option::Some(user_info))
            }

            async fn put(
                key: &str,
                value: #name,
            ) -> Result<#name, wasm_bindgen::__rt::std::io::Error> {
                let val = serde_json::to_string(&value)?;
                if let Ok(_) = Self::call_js(unsafe { #js_put(key, val.as_str()) }).await {
                    let res = Self::get(key).await?.unwrap();
                    Ok(res)
                } else {
                    Err(wasm_bindgen::__rt::std::io::Error::new(
                        wasm_bindgen::__rt::std::io::ErrorKind::NotFound,
                        format!("Could not find #name with id {}", key),
                    ))
                }
            }

            async fn delete(key: &str) -> Result<(), wasm_bindgen::__rt::std::io::Error> {
                match Self::call_js(unsafe { #js_delete(key) }).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }

        impl #name {
            async fn call_js(
                promise: js_sys::Promise,
            ) -> wasm_bindgen::__rt::std::result::Result<
                wasm_bindgen::JsValue,
                wasm_bindgen::__rt::std::io::Error,
            > {
                let result = wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map_err(|e| {
                        wasm_bindgen::__rt::std::io::Error::new(
                            wasm_bindgen::__rt::std::io::ErrorKind::Other,
                            e.as_string().unwrap(),
                        )
                    })?;
                Ok(result)
            }
        }

    };
    res.into()
}
