use yew::{html, Html};

pub fn home() -> Html {
    html! {
    <div>
        <p><em><strong>{"This is Marcus Rådell's site for things related to programming."}</strong></em></p>

            <section>
            <h2>{"What's Monadium.org?"}</h2>
            <p>{"This is where I point programmers and organizations who reach out to get my help."}</p>
            <p>{"It's also a community that I run where we make things a little better for developers and tech companies."}</p>
            </section>

            <section>
            <h2>{"For Companies"}</h2>
            <p>{"I'm a freelancing Director of Engineering who mostly work with startups and scaleups."}</p>
            <p>{"I can build apps, teams, and processes for the cost of "}<strong>{"1'000 SEK/h"}</strong><em>{" (ex. VAT)"}</em>{"."}</p>
            <p>{"I also help a selected set of talented programmers to find their dream job by pair programming and coaching them. If you want to hire a Monadium-approved candidate, the finder's fee is "} <strong>{"30'000 SEK"}</strong><em>{" (ex. VAT)"}</em>{"."}</p>
            </section>

            <section>
            <h2>{"For Individual Programmers"}</h2>
            <p>{"You can hire me to help with career advice, get better at product planning and coding, get a code review, pair program, or just talk."}</p>
            <p>{"I offer individuals who can't access my services through their company a cheaper price of "}<strong>{"500 SEK/h"}</strong><em>{" (ex. VAT)"}</em>{". "}</p>
            <p>{"The purpose of this offer is to enable more people in need to access my services. It means that I encourage those who can to hire me as a company coach at standard rate."}</p>
            </section>

            <section>
            <h2>{"Who am I?"}</h2>
            <p>{"My name is Marcus Rådell, and I'm a programmer who likes to help. I'm working mostly with startups and scaleups as a freelancing Director of Engineering, helping CTOs and CEOs with the harder parts of tech."}</p>
            <p>{"I usually coach teams in how to deliver value each week by keeping unimportant work away."}</p>
            <p>{"I do fullstack development including ideation, product planning, agile coaching, frontend, backend, and infrastructure. I lean towards Flutter for mobile apps, Rust for backends, GCP for cloud infra. I have most of my experience with TypeScript using React and node.js."}</p>
            <p>{"I usually work remotely from my home office in Stockholm. If you want me to swing by your place, please extend the invitation to my partner Moa, a 9-year old office-dog (boxer)."}</p>
            <p>{"You can find me on various sites as "}<strong>{"marcusradell"}</strong>{", with some exceptions."}</p>
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
            <h2>{"What's the best way to connect?"}</h2>
            <p>{"The Monadium.org Discord server: "} <a href="https://discord.gg/59hgZycxYJ">{"https://discord.gg/59hgZycxYJ"}</a>{"."}</p>
            <p>{"Throw us an introduction in the "}<strong>{"#introductions"}</strong>{" channel, or just talk in the "}<strong>{"#lounge"}</strong>{"."}</p>
            <p>{"Once inside, feel free to contact me in private or setup a video call."}</p>
            <p>{"You are free to use other methods of contact, but a proper chat server provides a whole lot of benefits for all parties involved. You should try it out!"}</p>
            </section>

            <section>
            <h2>{"Articles"}</h2>

            <ul>
            <li><a href="articles/hiring_programmers">{"Hiring Programmers"}</a></li>
            </ul>
            </section>

    </div> }
}
