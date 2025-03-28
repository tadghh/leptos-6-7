use leptos::prelude::*;
#[component]
pub fn ErrorNotification(error_text: String) -> impl IntoView {
    view! {
        <div class="error">
            <div>
                <p>Server Error: {error_text}</p>
            </div>
        </div>
    }
}
