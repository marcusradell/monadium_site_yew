use yew::{html, Html};

pub fn not_found() -> Html {
    html! {
        <div>
            <h2>{ "New page, who dis?" }</h2>
            <p>{"It never feels good to be stood up. I'm sorry 🙏"}</p>
            <p>{"Can we meet back home and talk about it?"}</p>
        </div>
    }
}
