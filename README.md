This is an example client server implementation using Yew, Tarpc, wsstreams (Server/Client wrappers for AsyncRead/Write byte streams) and Wasm-Timer shims. 


To build the server/rpc use: cargo build. 

To build the client use: wasm-pack build --target web --out-name wasm --out-dir ./static --debug


Start the client server using miniserve: 

miniserve ./static --index index.html --interfaces 0.0.0.0 --port 8083

Start the Server. 

Click Connect. 

Send pings, echos. 
