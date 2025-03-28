use leptos::{component, prelude::*, view, IntoView};
use leptos_meta::Title;
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Title text="Contact" />
        <div class="transfer-container">
            <div class="transfer-header">
                <span>"Contacts"</span>
            </div>
            <ul class="mt-3.5 text-3xl alink">
                <li>
                    <a href="https://www.linkedin.com/in/tadghh/">
                        <span>"LinkedIn 📋"</span>
                    </a>
                </li>
                <li>
                    <a href="mailto: ethhenry01@gmail.com">"ethhenry01@gmail.com 📧"</a>
                </li>
            </ul>
        </div>
    }
}
