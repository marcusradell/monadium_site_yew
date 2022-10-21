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
            <p>
                <ul>
                    <li>{"Twitter: "} <a href="https://twitter.com/marcusradell">{"marcusradell"}</a></li>
                    <li>{"LinkedIn: "}<a href="https://www.linkedin.com/in/marcusradell/">{"marcusradell"}</a></li>
                    <li>{"GitHub: "}<a href="https://github.com/marcusradell/">{"marcusradell"}</a></li>
                    <li>{"Reddit: "}<a href="https://www.reddit.com/user/ummonadi">{"ummonadi"}</a></li>
                </ul>
            </p>
            </section>

            <section>
            <h2>{"For Companies"}</h2>
            <p>{"I'm a freelancing Director of Engineering who mostly work with startups and scaleups."}</p>
            <p>{"I can build apps, teams, and processes for the cost of "}<strong>{"1'000 SEK/h"}</strong><em>{" (ex. VAT)"}</em>{"."}</p>
            <p>{"I also help a selected set of talented programmers to find their dream job by pair programming and coaching them. If you want to hire a Monadium-approved candidate, the finder's fee is "} <strong>{"30'000 SEK"}</strong><em>{" (ex. VAT)"}</em>{"."}</p>
            </section>

            <section>
            <h2>{"I want in!"}</h2>
            <p>{"Here's the Discord chat server: "} <a href="https://discord.gg/59hgZycxYJ">{"https://discord.gg/59hgZycxYJ"}</a>{"."}</p>
            <p>{"Throw us an introduction in the #introductions channel, or just talk in the #lounge."}</p>
            </section>

            <section>
            <h2>{"Articles"}</h2>

            <ul>
            <li><a href="articles/hiring_programmers">{"Hiring Programmers"}</a></li>
            </ul>
            </section>

    </div> }
}
