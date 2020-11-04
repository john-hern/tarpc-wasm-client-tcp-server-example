#![recursion_limit = "256"]

pub mod app;
pub mod loggin;
pub mod rpc_client;

use log::info;
use loggin::LogWriter;

use wasm_bindgen::prelude::*;

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
