mod app;
mod views;
mod data;
mod components;
mod utils;

use app::App;

fn main() {
     wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
