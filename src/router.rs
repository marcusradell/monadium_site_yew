use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::*;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/articles")]
    Articles,
    #[at("/articles/:id")]
    Article { id: String },
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
                    Route::Articles => articles(),
                    Route::Article {id} => article(id),
                    Route::NotFound => not_found()
                }
            }
            </div>
    }
}
