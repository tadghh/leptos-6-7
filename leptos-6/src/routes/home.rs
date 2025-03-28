use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Tadgh Ethan Henry" />
        <div class="intro-page-container">
            <section class="flex flex-col justify-between mt-2 home-section">
                <div class="flex justify-center my-80">
                    <div class="intro-container">
                        <h1>"Junior Software Developer / DevOps"</h1>
                        <span>"Manitoba, Canada 🇨🇦"</span>
                        <small>
                            <em>"The most Senior, Junior Developer"</em>
                        </small>
                    </div>
                </div>
                <div class="px-2.5 mt-60 mb-80">
                    <p class="mb-5 text-xl font-bold text-center">
                        "Thinking outside the box, creating abstract solutions to solve the "
                        <em>"impossible"</em>
                    </p>
                </div>
                <div class="flex flex-col mt-52">
                    <div class="intro-paragraph">
                        <h2>"About Me"</h2>
                        <p>
                            "I'm Ethan, a software/web developer whose passion began in Grade 5 when experimenting with computer scripts. Early experiences with low-performance computers inspired my commitment to creating efficient software. I graduated from Red River College's Business Information Technology program in 2023 with honors, and now I contribute to Open-Source Software while exploring interests in finance and psychology."
                        </p>
                    </div>
                </div>
            </section>
            <section class="mt-60 home-section">
                <h2 class="prj-pr-header">
                    <a href="/projects">"Project Showcase"</a>
                </h2>
                <ul class="prj-list">
                    <li>
                        <a class="prj-desc" href="https://github.com/tadghh/Shelf">
                            <h3>"Shelf:"</h3>
                            "A E-Reader developed with Rust and NextJS. This project helped me develop a basic understanding of Rust, along with the process of releasing an application to the public."
                        </a>
                        <ul>
                            <li>"Next.js"</li>
                            <li>"Rust"</li>
                            <li>"Tailwind"</li>
                            <li>"epubjs"</li>
                        </ul>
                    </li>
                    <li>
                        <a class="prj-desc" href="https://github.com/tadghh/python-weather-app">
                            <h3>"Environment Canada Web Scraper:"</h3>
                            "Its in the title, scrapes all weather data from the Environment Canada website. Initially this process took over 15 minutes, I reduced this down to 30 seconds by using async operations and multithreading* along with optimizing how the scraper would traverse through the websites HTML."
                        </a>
                        <ul>
                            <li>"Python"</li>
                            <li>"Pylint"</li>
                            <li>"GitHub Actions"</li>
                            <li>"Inno Setup"</li>
                        </ul>
                    </li>
                    <li>
                        <a
                            class="prj-desc"
                            href="https://github.com/tadghh/userstyles/tree/main/styles/Favicon%20Action%20area%20and%20Title%20Marquee"
                        >
                            <h3>"Styling Firefox with userChrome.css:"</h3>
                            "Customizing the look and feel, providing a more responsive and reactive experience. This required some creative problem solving partially because im not interested in updating the theme every time Mozilla makes changes to the user interface, but also creating animations based on a fixed set of elements while also managing state making sure the correct animation starts when Firefox is loading, playing music, muted etc."
                        </a>
                        <ul>
                            <li>"Advanced CSS Selectors"</li>
                            <li>"CSS Animations"</li>
                        </ul>
                    </li>
                </ul>
                <section class="mt-36 text-center overflow-clip rounded-md">
                    <a href="/links">
                        <h2 class="p-5 text-5xl font-bold c-main-nav_link">"Other Content ⤻"</h2>
                    </a>
                </section>
                <div class="mt-52">
                    <h2 class="text-4xl font-bold">"Skills"</h2>
                    <small class="text-base">
                        <em>
                            "Buzztown, home to the buzzwordians and AI generated job
                            descriptions"
                        </em>
                    </small>
                    <section class="flex flex-wrap justify-evenly mt-12">
                        {#[cfg(feature = "ssr")]
                        {
                            use crate::components::content::tags_content::Tags;
                            Tags()
                        }}
                    </section>
                </div>
            </section>
        </div>
    }
}
