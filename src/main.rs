#![recursion_limit = "512"]
use serde_derive::{Deserialize, Serialize};

use std::panic;

mod agent;
mod app;
mod components;
mod page;
mod service;

use app::App;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Config {
    pub base_url: String,
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
