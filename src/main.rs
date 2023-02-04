mod ui;
mod categorizer_service;
mod config;

use ui::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
