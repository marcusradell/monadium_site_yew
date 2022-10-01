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
                    Route::Home => home(),
                        Route::NotFound => not_found()
                }
            }
            </div>
    }
}
