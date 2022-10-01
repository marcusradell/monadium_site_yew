use yew::{html, Html};

pub fn home() -> Html {
    html! {
    <div>
        <p><em><strong>{"Helping hand for the junior developer."}</strong></em></p>

        <section>
            <h2>{"What's Monadium.org?"}</h2>
            <p>{"It's a community where we try to make things a little better for developers and tech companies."}</p>

            <h2>{"Who's running the show?"}</h2>
            <p>{"My name is Marcus Rådell, and I'm a programmer who likes to help."}</p>
            <p>{"You can find me as "}<strong>{"marcusradell"}</strong>{" on twitter, github, and linkedin. I'm "}<strong>{"ummonadi"}</strong>{" on reddit."}</p>

            <h2>{"I want in!"}</h2>
            <p>{"Here's the Discord chat server: "} <a href="https://discord.gg/59hgZycxYJ">{"https://discord.gg/59hgZycxYJ"}</a>{"."}</p>
            <p>{"Throw us an introduction in the #introductions channel, or just talk in the #lounge."}</p>

        </section>

    </div> }
}
