To build the client, run the following command: 

wasm-pack build --target web --out-name wasm --out-dir ./static --debug


Then to host the WASM. 

miniserve ./static --index index.html --interfaces 0.0.0.0 --port 8083

