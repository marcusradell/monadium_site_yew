use yew::{html, Html};

pub fn article() -> Html {
    html! {
        <div>
        <h1>{"Hiring Junies"}</h1>

        <p>{"This is a presentation on how we could optimize our teams for junior programmers and what that would mean for the rest of us."}</p>

        <section>
        <h2>{"Overview"}</h2>
        <p>
            <ol>
                <li>{"🔥 I Need To Hire"}</li>
                <li>{"👩‍💻 I Need A Job"}</li>
                <li>{"📶 Finding A Candidate"}</li>
                <li>{"🏢 Finding A Company"}</li>
                <li>{"🔎 Reviewing Code"}</li>
                <li>{"👯 Team Fit"}</li>
                <li>{"🚀 Onboarding"}</li>
                <li>{"⌚ Product Planning"}</li>
                <li>{"💾 Programming"}</li>
            </ol>
        </p>
        </section>

        <section>
        <h2>{"🔥 I Need To Hire"}</h2>

        <h3>{"Junior Programmer Not A Certified Title"}</h3>
        <p>
            {"Huge variation with a long tail of programmers who aren't there yet."}
        </p>

        <h3>{"Unclear claims of valuable skills"}</h3>
        <p>
            {r#""I went to a great programming school. Some students sucked, but I'm good.""#}
        </p>
        <p>
            {"Can you get credible references or list achievements that set you apart?"}
        </p>

        <p>
            {r#""Check out this movie app I did. It lists movies. I also have a TODO app. It's old though. I write much better code now.""#}
        </p>

        <p>
            {r#""I know JS, HTML, CSS, Figma, Git, VSCode, and how to send emails.""#}
        </p>

        <p>
            {"Few profiles show their top skills."}
        </p>
        </section>

        <section>
        <h2>{"👩‍💻 I Need A Job"}</h2>

        <h3>{"You Are Not A Junie"}</h3>
        <p>
        {"You are working hard as a programmer right now. Take ownership of your career. Work as a professional right now."}
        </p>
        </section>

        <section>
        <h2>{"📶 Finding A Candidate"}</h2>
        <p>
            {""}
        </p>
        </section>

        <section>
        <h2>{"🏢 Finding A Company"}</h2>
        <p>
            {""}
        </p>
        </section>

        <section>
        <h2>{"🔎 Reviewing Code"}</h2>
        <p>
            {""}
        </p>
        </section>

        <section>
        <h2>{"👯 Team Fit"}</h2>
        <p>
            {""}
        </p>
        </section>

        <section>
        <h2>{"🚀 Onboarding"}</h2>
        <p>
            {""}
        </p>
        </section>

        <section>
        <h2>{"⌚ Product Planning"}</h2>
        <p>
            {""}
        </p>
        </section>

        <section>
        <h2>{"💾 Programming"}</h2>
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
