pub mod rpc_client; 
pub mod loggin;


use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::rpc_client::build_client;
use ::rpc::services;
use futures::prelude::*;
use log::{info, error};
use tarpc::{
    client, context,
    server::{self, Channel, Handler},
};
use wasm_bindgen_futures::spawn_local;
use loggin::LogWriter;
//use wasm_timer::SystemTime;
use ::tarpc::trace;
use wasm_timer::{SystemTime as WasmTime, Instant};
use std::time::{Duration, UNIX_EPOCH};
use rand::RngCore;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
    msgs: Vec<String>
}

enum Msg {
    AddOne,
    Connect
}

/*
pub fn context() -> tarpc::context::Context {

    
    info!("Getting the current time");
    let wasm_time = WasmTime::now();
    let tmp = wasm_time.duration_since(wasm_timer::UNIX_EPOCH).unwrap();
    
    let sys_time = UNIX_EPOCH + std::time::Duration::from_secs(tmp.as_secs());
    info!("Got the current time {:?}", sys_time);
    let cx = tarpc::context::Context { 
        deadline: sys_time + Duration::from_secs(10),
        trace_context: trace::Context::new_root(),
    };
    info!("Returning the context");
    cx
}
*/

impl Model { 
    fn connect(&mut self) { 
        info!("Attemping to connect");
        spawn_local(Self::_connect());
    }
    async fn _connect() { 
        info!("Connecting");
        info!("Testing thread_rnd");
        let rng = &mut rand::thread_rng();
        let next = rng.next_u32();
        info!("Generated a random: {}", next);
        let transport = build_client();
        if let Ok(trans) = transport.await { 
            info!("Connected");
            let config = tarpc::client::Config::default();
            let client = services::PingServiceClient::new(config, trans);
            let dispatch = client
            .dispatch
            .unwrap_or_else(move |e| error!("Connection broken: {}", e));
            info!("Spawning Dispatch");
            spawn_local(dispatch);
            
            let mut client = client.client;
            info!("Creating Context");
            let cx = context::current();
            info!("Sending Ping!");
            let result = client.ping(cx).await.unwrap();
            if let Ok(msg) = result { 
                info!("Ping success: {}", msg);
            }
            let result = client.ping(cx).await.unwrap();
            if let Ok(msg) = result { 
                info!("Ping success: {}", msg);
            }
            let result = client.ping(cx).await.unwrap();
            if let Ok(msg) = result { 
                info!("Ping success: {}", msg);
            }
            //println!("Result:");
            
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
            msgs: Vec::new()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::Connect => self.connect(),
        }
        true
    }



    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let mut ret: String = "".into();
        for msg in &self.msgs {
            ret = ret + msg +"\r\n";  
        }
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                <p>{ self.value }</p>
                <div>{   
                    ret
                }</div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    let log_writer = LogWriter::new(log::Level::Info);
    if let Ok(()) = log::set_boxed_logger(Box::new(log_writer)) {
        log::set_max_level(log::LevelFilter::Info)
    }
    info!("First Message");
    App::<Model>::new().mount_to_body();
}