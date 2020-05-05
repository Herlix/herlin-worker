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
    let get = Ident::new(&format!("js_get_{}", &name), ast.ident.span());
    let put = Ident::new(&format!("js_put_{}", &name), ast.ident.span());
    let delete = Ident::new(&format!("js_delete_{}", &name), ast.ident.span());

    let res = quote! {
        impl crate::kv::JsKvPromise for #name {
            fn js_get(key: &str) -> js_sys::Promise {
                #get(key)
            }

            fn js_delete(key: &str) -> js_sys::Promise {
                #delete(key)
            }

            fn js_put(key: &str, value: &str) -> js_sys::Promise {
                #put(key, value)
            }
        }

        #[wasm_bindgen::prelude::wasm_bindgen]
        extern "C" {
            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = get)]
            fn #get(key: &str) -> js_sys::Promise;

            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = put)]
            fn #put(key: &str, value: &str) -> js_sys::Promise;

            #[wasm_bindgen::prelude::wasm_bindgen(js_namespace = #name, js_name = delete)]
            fn #delete(key: &str) -> js_sys::Promise;
        }

       #[async_trait::async_trait(?Send)]
        impl crate::kv::CloudFlareKV<'_, #name> for #name
        {
            async fn get(key: &str) -> Result<#name, wasm_bindgen::__rt::std::io::Error> {
                let result = crate::kv::call_js(Self::js_get(key))
                    .await?
                    .as_string()
                    .unwrap();
                let result = serde_json::from_str(result.as_str())?;
                Ok(result)
            }

            async fn put(
                key: &str,
                value: #name,
            ) -> Result<#name, wasm_bindgen::__rt::std::io::Error> {
                let value = serde_json::to_string(&value)?;
                if let Ok(_) = crate::kv::call_js(Self::js_put(key, value.as_str())).await {
                    Self::get(key).await
                } else {
                    Err(wasm_bindgen::__rt::std::io::Error::new(
                        wasm_bindgen::__rt::std::io::ErrorKind::NotFound,
                        format!("Could not find #name with id {}", key),
                    ))
                }
            }

            async fn delete(key: &str) -> Result<(), wasm_bindgen::__rt::std::io::Error> {
                match crate::kv::call_js(Self::js_delete(key)).await {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
        }

    };
    res.into()
}
