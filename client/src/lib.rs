pub mod app;
pub mod loggin;
pub mod rpc_client;

use crate::rpc_client::build_client;
use ::rpc::services;
use ::tarpc::trace;
use futures::prelude::*;
use log::{error, info};
use loggin::LogWriter;
use rand::RngCore;
use std::time::{Duration, UNIX_EPOCH};
use tarpc::{
    client, context,
    server::{self, Channel, Handler},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_timer::{Instant, SystemTime as WasmTime};
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    let log_writer = LogWriter::new(log::Level::Info);
    if let Ok(()) = log::set_boxed_logger(Box::new(log_writer)) {
        log::set_max_level(log::LevelFilter::Info)
    }
    info!("First Message");
    App::<crate::app::Model>::new().mount_to_body();
}
