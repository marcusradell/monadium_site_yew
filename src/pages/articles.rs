use yew::{html, Html};

pub fn articles() -> Html {
    html! {
        <div>
            <h2>{"Articles"}</h2>

            <ul>
                <li>
                    <a href="/articles/testing_article_setup">{"Testing article setup"}</a>
                </li>
            </ul>
        </div>
    }
}
