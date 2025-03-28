use leptos::{component, view, IntoView};
use leptos_meta::Title;

#[component]
pub fn SocialLinks() -> impl IntoView {
    view! {
        <Title text="Other Content" />
        <div class="transfer-container">
            <div class="transfer-header">
                <span>"Other Content"</span>
            </div>
            <div class="transfer-suggestions">
                <ul>
                    <li>
                        <a href="/blog">"Blog ✍️"</a>
                    </li>
                    <li>
                        <a href="/projects">"Projects 🔨"</a>
                    </li>
                    <li>
                        <a href="https://github.com/tadghh">"GitHub 🐙"</a>
                    </li>
                    <li>
                        <a href="https://www.linkedin.com/in/tadghh/">"LinkedIn 📋"</a>
                    </li>
                </ul>
            </div>
        </div>
    }
}
