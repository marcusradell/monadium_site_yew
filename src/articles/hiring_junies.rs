use yew::{html, Html};

pub fn article() -> Html {
    html! {
        <div>
        <h1>{"Hiring Junies"}</h1>

        <section>
        <h2>{"Overview"}</h2>
        <p>
            <ul><li>{"I Need To Hire 🔥"}</li></ul>
            <ul><li>{"I Need A Job 👩‍💻"}</li></ul>
            <ul><li>{"Reviewing Code 🔎"}</li></ul>
            <ul><li>{"Team Fit 👯"}</li></ul>
        </p>
        </section>

        <section>
        <h2>{"I Need To Hire 🔥"}</h2>

        <h3>{"Junior Programmer Not A Certified Title"}</h3>
        <p>
            {"Huge variation with a long tail of programmers who aren't there yet."}
        </p>

        <h3>{"Unclear value propositions"}</h3>
        <p>
            {"Educational background the main claim."}
        </p>

        <p>
            {"Most profiles try to show a width of skills."}
        </p>

        <p>
            {"Few profiles show their top skills."}
        </p>
        </section>

        <section>
        <h2>{"I Need A Job 👩‍💻"}</h2>
        <p>
            {""}
        </p>
        </section>

        <section>
        <h2>{"The Expertiment 🧪"}</h2>
        <p>
            {""}
        </p>
        </section>

        <section>
            <div>{"Marcus Rådell"}</div>
            <div>{"2022-10-06"}</div>
        </section>
        </div>
    }
}
