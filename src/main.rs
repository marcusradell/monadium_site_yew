use yew::prelude::*;
use yew_router::prelude::*;

mod router;
use router::*;

mod pages;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
