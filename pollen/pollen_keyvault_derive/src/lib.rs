#[warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
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

        impl pollen::CloudFlareKV for #name {
            fn get<'a>(key: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, pollen::JSError>> + 'a>>
            where
                Self: serde::de::DeserializeOwned + 'a,
            {
                async fn run<'a>(key: &'a str) -> Result<#name, pollen::JSError>
                {
                    let result = #name::await_js(#js_get(key)).await?;
                    Ok(serde_json::from_str::<#name>(
                        result.as_string().unwrap().as_str(),
                    )?)
                }

                Box::pin(run(key))
            }

            fn put<'a>(key: &'a str, value: Self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, pollen::JSError>> + 'a>>
            where
                Self: serde::Serialize + serde::de::DeserializeOwned + 'a,
            {
                async fn run<'a>(key: &'a str, value: #name) -> Result<#name, pollen::JSError>
                {
                    let val = serde_json::to_string(&value)?;
                    if let Ok(_) = #name::await_js(#js_put(key, val.as_str())).await {
                        let result = #name::await_js(#js_get(key)).await?;
                        Ok(serde_json::from_str::<#name>(
                            result.as_string().unwrap().as_str(),
                        )?)
                    } else {
                        Err(pollen::JSError {
                            msg: format!("Could not find #name with id {}", key),
                        })
                    }
                }

                Box::pin(run(key, value))
            }

            fn delete<'a>(key: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), pollen::JSError>> + 'a>> {
                async fn run<'a>(key: &'a str) -> Result<(), pollen::JSError> {
                    match #name::await_js(#js_delete(key)).await {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e),
                    }
                }

                Box::pin(run(key))
            }
        }

        #[wasm_bindgen::prelude::wasm_bindgen]
        extern "C" {
            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = get)]
            fn #js_get(key: &str) -> js_sys::Promise;

            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = put)]
            fn #js_put(key: &str, value: &str) -> js_sys::Promise;

            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = delete)]
            fn #js_delete(key: &str) -> js_sys::Promise;
        }

        impl #name {
            async fn await_js(promise: js_sys::Promise) -> Result<wasm_bindgen::JsValue, pollen::JSError> {
                let result = wasm_bindgen_futures::JsFuture::from(promise).await.map_err(|e| pollen::JSError {
                    msg: e.as_string().unwrap(),
                })?;
                Ok(result)
            }
        }

    };
    res.into()
}
