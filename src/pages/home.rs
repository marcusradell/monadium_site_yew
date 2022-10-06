use yew::{html, Html};

pub fn home() -> Html {
    html! {
    <div>
        <p><em><strong>{"Helping hand for the junior developer."}</strong></em></p>

            <section>
            <h2>{"What's Monadium.org?"}</h2>
            <p>{"It's a community where we try to make things a little better for developers and tech companies."}</p>
            </section>

            <section>
            <h2>{"Who's running the show?"}</h2>
            <p>{"My name is Marcus Rådell, and I'm a programmer who likes to help."}</p>
            <p>{"You can find me as "}<strong>{"marcusradell"}</strong>{" on twitter, github, and linkedin. I'm "}<strong>{"ummonadi"}</strong>{" on reddit."}</p>
            </section>

            <section>
            <h2>{"What can you offer?"}</h2>
            <p>{"I'm a freelancing Director of Engineering who mostly work with startups and scaleups."}</p>
            <p>{"I can build apps, teams, and processes for the cost of "}<strong>{"1'000 SEK/h"}</strong><em>{" (ex. VAT)"}</em>{"."}</p>
            <p>{"I also dabble in recruitment where I do technical interviews by pair programming with candidates."}</p>
            </section>

            <section>
            <h2>{"I want in!"}</h2>
            <p>{"Here's the Discord chat server: "} <a href="https://discord.gg/59hgZycxYJ">{"https://discord.gg/59hgZycxYJ"}</a>{"."}</p>
            <p>{"Throw us an introduction in the #introductions channel, or just talk in the #lounge."}</p>
            </section>

            <section>
            <h2>{"Articles"}</h2>

            <ul>
            <li><a href="articles/hiring_junies">{"Hiring Junies"}</a></li>
            </ul>
            </section>

    </div> }
}
