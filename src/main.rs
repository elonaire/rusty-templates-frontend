mod app;
mod views;
mod data;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
