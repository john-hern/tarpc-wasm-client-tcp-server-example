
use crate::rpc_client::build_client;
use ::rpc::services;

use futures::prelude::*;
use log::{error, info};
use rand::RngCore;

use tarpc::{
    client, context,
    server::{self, Channel, Handler},
};

use wasm_bindgen_futures::spawn_local;
//use wasm_timer::{Instant, SystemTime as WasmTime};
use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
    connected: bool,
}

pub enum Msg {
    AddOne,
    Connect,
    Ping,
    Echo,
}

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
            connected: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::Connect => self.connect(),
            Msg::Ping => (),
            Msg::Echo => (),
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
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                <p>{ self.value }</p>
                <div>{"Connected: "}{
                    if self.connected {
                        "True"
                    }else {
                        "False"
                    }
                }</div>
            </div>
        }
    }
}
