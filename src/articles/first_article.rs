use yew::{html, Html};

pub fn article() -> Html {
    html! {
        <div>
        <h2>{"First Article"}</h2>

        <section>
            {"Just trying things out to see how it looks."}
        </section>

        <section>
            {"Just trying things out to see how it looks."}
        </section>

        <section>
            <div>{"Marcus Rådell"}</div>
            <div>{"2022-09-02"}</div>
        </section>
        </div>
    }
}
