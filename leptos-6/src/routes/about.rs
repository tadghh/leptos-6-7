use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About" />
        <div class="about-page-cont">
            <h1 class="page-title">"About"</h1>
            <div class="about-sec">
                <h3>"My Name"</h3>
                <p class="leading-4">
                    "I prefer to go by my middle name, Ethan. This is thanks to the cumulative amount of time it takes to explain my first name. If you are curious about the phonetics here’s a short guide, my first name is ‘Tadgh’ (pronounced tye-gh), thanks to its Gaelic origins, the ‘a’ makes a ‘ye’ sound, the ‘d’ is silent and it ends with a hard ‘g’. Overall, it’s just inefficient and inaccessible; not a good use of time"
                </p>
            </div>

            <div class="about-sec-contain work-exp">
                <h3>"Work Experience"</h3>

                <div class="mb-3">
                    <h3>"Junior DevOps / Associate Consultant - Optimiz"</h3>
                    <span>"May 2023 - Aug 2023"</span>
                    <p>
                        "Integrated Ansible into internal tooling, reducing deployment time of AppDynamics agents by 80% across various hosts."
                    </p>
                    <ul>
                        <li>"Resolved bi-weekly WordPress reliability issues"</li>
                        <li>"Technologies used: Grafana, Selenium, Azure, Express.js"</li>

                    </ul>
                </div>
                <div>
                    <h3>"Junior Full Stack Developer - Red River College Polytech"</h3>
                    <span>"Jan 2023 - Apr 2023"</span>
                    <p>
                        "Developed a web application to provide customers with on demand services. Using the Agile methodology my team an I worked alongside a client allowing for flexible design and consistent feedback. Additionally teaching alongside 4 other junior developers, helping them learn and work through difficult tasks."
                    </p>
                    <ul>
                        <li>"Implemented user authentication via NextAuth"</li>
                        <li>
                            "Built out an API to handle account admin tasks like, password resets, and verification emails sent during account
                             creation or for password resets"
                        </li>
                        <li>"Built authenticated REST APIs to handle user information"</li>
                        <li>
                            " Developed web form validation schemas, implementing them on
                             frontend and backend. Preventing the storage of incorrect user data."
                        </li>
                        <li>
                            "Wrote 7 pages of documentation, covering implementation and
                            testing"
                        </li>
                        <li>"Technologies used: Next.js and PostgreSQL, Prisma ORM"</li>
                    </ul>
                </div>
            </div>
            <div class="about-sec-contain">
                <h3>"Education"</h3>
                <div>
                    <h3 class="text-lg font-bold leading-4">
                        "Red River College Polytechnic: Business Information Technology"
                    </h3>
                    <span class="text-sm font-light tracking-tighter">"Aug 2021 - Dec 2023"</span>
                    <p class="ml-5 text-sm">"GPA: 4.35/4.5"</p>
                </div>
            </div>
            <div class="about-sec-contain">
                <h3>"Earlier Work - "<small>"Before College"</small></h3>
                <dl>
                    <dt class="leading-4">"2021 - Minimal Windows 10 Image"</dt>
                    <dd>
                        "Created Windows Install images using DISM (Deployment Image Servicing and Management) improving performance on low end hardware. With a 10GB install footprint these images were 1/4 the size of a typical install, additionally these reductions also brought down the idle RAM usage to 800MB, a regular install hovers around 3GB of RAM. Due to the restrictive requirements of this project I learned new ways of troubleshooting issues, and an understanding of the Windows Installation Media internals and Windows 10."
                    </dd>
                    <dt>"2021 - Mod Development Kit User Interface"</dt>
                    <dd>
                        "Used Swing and Java to develop a user interface for a development tool. This tool was used to create community content for of the sandbox game, Garry’s Mod. From this project I learned about the importance of user experience and user interface design."
                    </dd>
                    <dt>"2020 - Android Debugging Bridge User Interface"</dt>
                    <dd>
                        "In Grade 12, I learned Java and Swing. My final project was to build a user interface for the Android Debugging Bridge (ADB), simplifying the management of system applications and complex command-line tasks to improve workflow and accessibility."
                    </dd>
                    <dt>"2014 - Adding Dark Mode to a VOIP Application"</dt>
                    <dd>
                        "Reverse engineered a popular Voice over IP (VOIP) application, adding a dark mode through the modification of embedded resource files."
                    </dd>
                </dl>
            </div>
        </div>
    }
}
