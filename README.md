# 👷‍♀️🦀🕸️ herlin-worker [![Build Status](https://travis-ci.com/Herlix/herlin-worker.svg?branch=master)](https://travis-ci.com/Herlix/herlin-worker)

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

## 🚴 Usage

### 🧗‍ Setup
````
Target: 
    rustup target add wasm32-unknown-unknown
Wasm helper tool: 
    https://rustwasm.github.io/wasm-pack/installer/#
Wrangler, Cloudflare CLI: 
    https://developers.cloudflare.com/workers/tooling/wrangler/install/
````

### 🛠️ Build & test
```
/* Build, test non wasm (unit tests), test wasm (integration tests)*/
cargo build && cargo test --all && wasm-pack test --node
```

### 🔧 Preview with `wrangler preview`
```
wrangler preview --watch
```

### ✅ Deployment
```
wrangler publish
```

