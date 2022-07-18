mod app;
mod components;
mod errors;
mod pages;
mod requests;

use app::App;
use errors::{InternalResponseError, MoolahFrontendError};

pub type ResponseResult<T> = Result<T, InternalResponseError>;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    yew::start_app::<App>();
}
