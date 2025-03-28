use leptos::{component, view, IntoView};
use leptos_meta::Title;

/// 404 - Not Found
#[component]
pub fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        use leptos::expect_context;

        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="uWupsie!" />
        <div class="transfer-container">
            <div class="transfer-header">
                <span>"404"</span>
            </div>

            <div class="transfer-suggestions">
                <ul>
                    <li>
                        <a href="/">"Home 🏠"</a>
                    </li>
                    <li>
                        <a href="/blog">"Blog ✍️"</a>
                    </li>
                    <li>
                        <a href="/projects">"Projects 🔨"</a>
                    </li>
                    <li>
                        <a href="/about">"About/Resume 💼"</a>
                    </li>
                    <li>
                        <a href="https://doc.rust-lang.org/book/">"Learn Rust 🦀"</a>
                    </li>
                </ul>
                <span class="block mt-24 text-lg text-center text-rose-200">
                    "The requested item was not found"
                </span>
            </div>
        </div>
    }
}
