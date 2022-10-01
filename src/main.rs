use yew::prelude::*;
use yew_router::prelude::*;

mod pages;

mod router;
use router::*;

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
