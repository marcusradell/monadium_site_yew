use yew::{html, Html};

pub fn article() -> Html {
    html! {
        <div>
        <h1>{"Hiring Programmers"}</h1>

        <p>{"This is a presentation on how we could optimize our programming teams, with focus on hiring and onboarding."}</p>

        <section>
        <h2>{"Overview"}</h2>
        <p>
            <ol>
                <li>{"🔥 I Need To Hire"}</li>
                <li>{"📝 The Job Ad"}</li>
                <li>{"👩‍💻 I Need A Job"}</li>
                <li>{"🔎 Reviewing Code"}</li>
                <li>{"🏀 The Interview Process"}</li>
                <li>{"🚀 Onboarding"}</li>
            </ol>

            <h3>{"Extras"}</h3>
            <ul>
                <li>{"⌚ Product Planning"}</li>
                <li>{"💾 Programming"}</li>
                <li>{"👵 From Junior To Senior"}</li>
            </ul>
        </p>
        </section>

        <section>
            <h2>{"🔥 I Need To Hire"}</h2>

            <p>{"Do you have an environment where your staff can thrive?"}</p>

            <p>{"Fix team baggage."}</p>

            <p>{"Make your process an awesome experience!"}</p>

            <p>{"Every candidate is part of your tribe."}</p>

            <p>{"Be genuine! Show what is broken."}</p>

            <p>{"Give feedback during the process."}</p>

            <p>{"Don't waste anyone's time or energy."}</p>

            <p>{"Don't require what you cannot offer."}</p>

            <p>{"Summary: "}<strong>{"Show empathy"}</strong>{"."}</p>
        </section>

        <section>
            <h2>{"📝 The Job Ad"}</h2>

            <p>{"Years of experience is overused."}</p>

            <p>{"Use your team's tonality."}</p>

            <p>{"Describe your team's culture."}</p>

            <p>{"What problems do you want solved?"}</p>

            <p>{"What matters most? Top 3."}</p>

            <p>{"Be active in your communities."}</p>

            <p>{"Show of your team."}</p>

            <p>{"Be uncomfortably transparent."}</p>
        </section>

        <section>
            <h2>{"👩‍💻 I Need A Job"}</h2>

            <p>{"Junior programmer is not a certified title."}</p>

            <p><img src="/static/longtail_graph.png" alt="Longtail graph" /></p>

            <p>{"Some extreme talents and some who aren't there "}<strong>{"yet"}</strong>{"."}</p>

            <p>{"Be professional and own your career."}</p>

            <p>{"Solve real problems."}</p>

            <p>{"Work with a project board."}</p>

            <p>{"Commit messages should come easy."}</p>

            <p>{"Learn to document."}</p>

            <p>{"Go deep on programming fundamentals."}</p>

            <p>{"Become an expert at something you enjoy."}</p>

            <p>{"It should be obvious that you are hireable."}</p>
        </section>

        <section>
            <h2>{"🔎 Reviewing Code"}</h2>

            <p>{"Less is more."}</p>

            <p>{"Attention for details."}</p>

            <p>{"Demo the product."}</p>

            <p>{"Document the solution."}</p>

            <p>{"What skills do you want to prove?"}</p>

            <p>{"Lead the reviewer."}</p>
        </section>

        <section>
            <h2>{"🏀 The Interview Process"}</h2>

            <p>{"First-contact reply within a day."}</p>

            <p>{"2-3 sessions."}</p>

            <p>{"Make them feel safe."}</p>

            <p>{"Present company, product, and team."}</p>

            <p>{"Collaborate based on their interest."}</p>

            <p>{"Extract value from churned candidates."}</p>
        </section>

        <section>
            <h2>{"🚀 Onboarding"}</h2>

            <p>{"Equipment and logins ready on first day."}</p>

            <p>{"Flowers or swag."}</p>

            <p>{"Record a greeting video."}</p>

            <p>{"Pair- or mob-program."}</p>

            <p>{"Prepare fallback project."}</p>

            <p>{"Learning by doing."}</p>
        </section>

        <section>
            <h2>{"⌚ Product Planning"}</h2>

            <p>{"Weekly cycle Mon-Fri with wrap-up on Mondays."}</p>

            <p>{"Plan for 2 weeks of work."}</p>

            <p>{"Remove low-prio work."}</p>

            <p>{"Wrap up: do releases and record demos."}</p>

            <p>{"Set weekly focus."}</p>
        </section>

        <section>
            <h2>{"💾 Programming"}</h2>

            <p>{"Add tickets as you discover work."}</p>

            <p>{"Learn to pair program properly."}</p>

            <p>{"Micro-steps. Rotate each commit."}</p>
        </section>

        <section>
            <h2>{"👵 From Junior To Senior"}</h2>

            <p>{"Juniors can code a feature. [Code]"}</p>

            <p>{"Get a mentor."}</p>

            <p>{"Optimize for learning."}</p>

            <p>{"Mid-level can plan a product. [Processes]"}</p>

            <p>{"Learn to think in TDD."}</p>

            <p>{"Set expectations."}</p>

            <p>{"Senior can facilitate a team. [Communication]"}</p>

            <p>{"Build social currency."}</p>

            <p>{"It's not about you anymore."}</p>

            <p>{"Delegate the hardest problems."}</p>
        </section>

        <section>
            <div>{"Marcus Rådell"}</div>
            <div>{"2022-10-06"}</div>
        </section>
        </div>
    }
}
