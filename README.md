# 👷‍♀️🦀🕸️ herlin-worker [![Build Status](https://travis-ci.com/Herlix/herlin-worker.svg?branch=master)](https://travis-ci.com/Herlix/herlin-worker)

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

## 🚴 Usage

### 🛠️ Build `wasm-pack`
```
wasm-pack build
```

### 🔧 Preview with `wrangler preview`
```
wrangler preview --watch
```

### 🔬 Test using node `wasm-pack test`
```
wasm-pack test --node
```

### ✅ Deployment
```
wrangler publish
```

