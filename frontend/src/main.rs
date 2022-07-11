mod app;
mod components;
mod errors;
mod pages;
mod services;

use app::App;
use errors::MoolahFrontendError;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    yew::start_app::<App>();
}
