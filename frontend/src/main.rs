mod app;
mod components;
mod errors;
mod pages;
mod services;

use app::App;
use errors::MoolahFrontendError;

fn main() {
    yew::start_app::<App>();
}
