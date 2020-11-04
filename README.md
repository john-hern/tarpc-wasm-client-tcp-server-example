This is an example client server implementation using Yew, Tarpc, wsstreams (Server/Client wrappers for AsyncRead/Write byte streams) and Wasm-Timer shims. 

Due to the way that Cargo builds in the root, we have to build everything explicitly. See Issue: https://github.com/rust-lang/cargo/issues/7004

To build the server/rpc use: 

cargo build -p rpc

cargo build -p server 


To build the client use: wasm-pack build --target web --out-name wasm --out-dir ./static --debug

Start the client server using miniserve: 

miniserve ./static --index index.html --interfaces 0.0.0.0 --port 8083

Start the Server. 

Click Connect. 

Send pings, echos. 
