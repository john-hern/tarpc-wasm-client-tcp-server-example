
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

use std::rc::Rc;
use std::cell::RefCell;

#[derive( Clone, Debug)]
pub struct Model {
    link: ComponentLink<Self>,
    value: i64,
    client: Rc<RefCell<Option<services::RPCServiceClient>>>,
    echo_value: String,
    echo_result: String,
    connected: bool
}

pub enum Msg {
    AddOne,
    Connect,
    Connected,
    Ping,
    UpdateEcho(String),
    UpdateEchoResult(String),
    Echo,
    Redraw,
}

impl Model {
    fn connect(&mut self) {
        info!("Attemping to connect");
        let client_ptr = self.client.clone();
        let link = self.link.clone();
        let fut = async move { 
            info!("Connecting");
            let transport = build_client();
            if let Ok(trans) = transport.await {
                info!("Connected");
                let config = tarpc::client::Config::default();
                let client = services::RPCServiceClient::new(config, trans);
                let dispatch = client
                    .dispatch
                    .unwrap_or_else(move |e| error!("Connection broken: {}", e));
                info!("Spawning Dispatch");
                
                //Spawn the dispatch future
                spawn_local(dispatch);
                
                //Store the client. 
                client_ptr.replace(Some(client.client));

                //Force the dom view to refresh to update the Connected status. 
                link.send_message(Msg::Connected);
            }
        };
        spawn_local(fut);
    }
    fn ping(&self) { 
        if self.connected{ 
            let client = self.client.clone();
            let fut = async move { 
                if let Some(ref mut client) = *client.borrow_mut() { 
                    let result = client.ping(context::current()).await.unwrap();
                    if let Ok(msg) = result {
                        info!("Ping success: Results {}", msg);
                    }
                }
            };
            spawn_local(fut);
        }
    }

    fn echo(&self, value: String)
    { 
        if self.connected{ 
            let client = self.client.clone();
            let link = self.link.clone();
            let fut = async move { 
                if let Some(ref mut client) = *client.borrow_mut() { 
                    let result = client.echo(context::current(), value).await.unwrap();
                    if let Ok(msg) = result {
                        info!("Echo Success: Results {}", msg);
                        link.send_message(Msg::UpdateEchoResult(msg));
                    }
                }
            };
            spawn_local(fut);
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
            client: Rc::new(RefCell::new(None)),
            echo_value: "".into(),
            echo_result: "Type string in input and press Echo".into(),
            connected: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::Connect => self.connect(),
            Msg::Ping => self.ping(),
            Msg::UpdateEcho(value) => self.echo_value = value,
            Msg::Echo => self.echo(self.echo_value.clone()),
            Msg::Redraw => (),
            Msg::UpdateEchoResult(result) => {
                info!("Updating the echo result");
                self.echo_result = result.clone();
            },
            Msg::Connected => self.connected = true
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
        let echo_result = self.echo_result.clone();
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <button onclick=self.link.callback(|_| Msg::Connect)>{ "Connect" }</button>
                <button onclick=self.link.callback(|_| Msg::Ping)>{ "Ping" }</button>
                <div>
                    <input
                        type = "text"
                        placeholder="Echo String"
                        value=&self.echo_value
                        oninput= self.link.callback(|e: InputData| Msg::UpdateEcho(e.value))
                    />
                    <button onclick=self.link.callback(|_| Msg::Echo)> { "Echo"} </button>
                    <div>{"Echoed Result: "}{echo_result} </div>
                </div>
                <p>{ self.value }</p>
                <div>
                {"Connected: "}{
                    if self.connected {
                        "True"
                    }else {
                        "False"
                    }
                }
                </div>
            </div>
        }
    }
}
