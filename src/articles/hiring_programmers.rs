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
                <li>{"🏀 The Interview Process"}</li>
                <li>{"🔎 Reviewing Code"}</li>
                <li>{"👯 Team Compatability"}</li>
                <li>{"🚀 Onboarding"}</li>
            </ol>

            <h3>{"Extras"}</h3>
            <ul>
                <li>{"⌚ Product Planning"}</li>
                <li>{"💾 Programming"}</li>
                <li>{"Road To Becoming Senior"}</li>
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

        <p>{"Years of experience is overused and mostly relevant at 5+ years."}</p>

        <p>{"Make your team culture a part of the job ad. Let the tonality reflect what they can expect at work."}</p>

        <p>{"Describe your culture explicitly. Who would thrive at your place. Who wouldn't like it?"}</p>

        <p>{"Describe what problems the applicant should be able to solve."}</p>

        <p>{"Describe your dream candidate, but only ask for the things that matters most."}</p>

        <p>{"Walk the walk. Open source what you can and let the code reflect your processes. Sponsor important packages. Be active in your communities."}</p>

        <p>{"The team matters the most. Let the team show who they are. Record videos, share personal interests, github- and linkedin-profiles, etc."}</p>

        <p>{"Be transparent with what you can such as salary expectations and interview processes."}</p>
        </section>

        <section>
        <h2>{"👩‍💻 I Need A Job"}</h2>

        <p>{"Junior programmer is not a certified title. There are lots of juniors out there."}</p>

        <p>
            <img src="/static/longtail_graph.png" alt="Longtail graph" />
        </p>

        <p>
            {"Large variation of skills with a long tail of programmers who aren't there "}<strong>{"yet"}</strong>{"."}
        </p>

        <p>{"Take ownership of your career. Work as a professional before your first job to get your first job."}</p>

        <p>{"Learn to break an app apart into small pieces. Work with a project board. Focus on one task at a time. Do not commit in code unrelated to the current task. Create new tasks and do them later. You should have clear commit messages if you work focused."}</p>

        <p>{"Learn to document. It's a great way to show that you are able to communicate with others."}</p>

        <p>{"Learn programming fundamentals. Take a course in your programming language of your choice. Learn to tidy up your code. Most code should look simple and obvious."}</p>

        <p>{"Become an expert at something."}</p>

        <p>{"You become hireable when it's obvious that you can contribute to a company."}</p>
        </section>

        <section>
        <h2>{"🏀 The Interview Process"}</h2>

        <p>{"First contact within a day."}</p>

        <p>{"Signing the contract within 1-2 weeks should be possible, but not required."}</p>

        <p>{"Two longer sessions should be enough, with a third optional one if any party wants to get more answers sorted out."}</p>

        <p>{"Add an ice breaker at the start of each session. The goal is to make candidates less nervous. It's your responsibility to make candidates feel safe."}</p>

        <p>{"Most processes will churn. Make sure they still bring value."}</p>
        </section>

        <section>
        <h2>{"🔎 Reviewing Code"}</h2>


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
            {r#""Here's my laboration code from my Udemy course.""#}
        </p>
        <p>
            {"Avoid copied content or laborations as proof of skills. They are fine as proof that you took the course."}
        </p>

        <p>
            {r#""I know JS, HTML, CSS, Figma, Git, VSCode, and how to send emails.""#}
        </p>

        <p>{"You need to say what you can achieve with the skills you have. What kind of problems can you solve with your programming skills?"}</p>

        <h3>{"Lead the way to your proof of claims"}</h3>

        <p>{""}</p>

        </section>

        <section>
        <h2>{"👯 Team Compatability"}</h2>
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
            {"I will assume a weekly product planning cycle that starts on Monday, ends on Friday, and is released next Monday."}
        </p>

        <p>
            {"Go through your weekly work board. It can contain 2-4 weeks of work. But discussing more than two weeks of work in detail is wasteful and can reduce the teams agility. Practice parking discussions that are not valuable to the entire team."}
        </p>

        <p>
            {"Remove work that you can't do in that time frame. Be smart with how you remove things without insulting external contributors."}
        </p>

        <p>
            {"Do a weekly demo. Record the screen, take images, write explanations. Everything should be demonstrated in an environment that is as close to production as possible."}
        </p>

        <p>
            {"Set a clear weekly focus. You can estimate what you can achieve internally in the team, but only describe in what direction you are focusing to those outside the team."}
        </p>
        </section>

        <section>
        <h2>{"💾 Programming"}</h2>
        <p>
            {"Add tickets as you discover work."}
        </p>

        <p>
            {"Learn how to pair program properly with a driver and navigator. Try to avoid sight seeing."}
        </p>
        </section>


        <section>
            <div>{"Marcus Rådell"}</div>
            <div>{"2022-10-06"}</div>
        </section>
        </div>
    }
}
