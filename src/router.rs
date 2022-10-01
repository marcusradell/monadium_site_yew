use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

pub fn switch(route: Route) -> Html {
    html! {
            <div class="content">
            <header>
            <h1>
                <a href="/" class="logo-link">{"🧡"}</a>
                { " Monadium.org" }
            </h1>
            </header>
            {
                match route {
                    Route::Home => {
                                html! {
                                <div>
                                    <p><em><strong>{"Helping hand for the junior developer."}</strong></em></p>
                                    <h2>{"Join Us!"}</h2>
                                    <p>{"Discord: "} <a href="https://discord.gg/59hgZycxYJ">{"https://discord.gg/59hgZycxYJ"}</a></p>
                                </div> }
                            }
                        Route::NotFound => html! {
                            <div>
                                <h2>{ "New page, who dis?" }</h2>
                                <p>{"It never feels good to be stood up. I'm sorry 🙏"}</p>
                                <p>{"Can we meet back home and talk about it?"}</p>
                            </div>
                        }
                }
            }
            </div>
    }
}
