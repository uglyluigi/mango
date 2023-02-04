mod ui;
use ui::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
