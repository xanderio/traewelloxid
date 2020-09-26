#![recursion_limit = "512"]

mod agent;
mod app;
mod components;
mod page;
mod service;
mod types;

use app::App;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
