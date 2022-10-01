use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
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
                        Route::NotFound => not_found()
                }
            }
            </div>
    }
}
