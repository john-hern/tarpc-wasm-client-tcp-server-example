wasm-pack build --target web --out-name wasm --out-dir ./static --debug


miniserve ./static --index index.html --interfaces 0.0.0.0 --port 8083

