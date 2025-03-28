use leptos::{component, view, IntoView};

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="flex justify-between items-center mx-1 mt-auto space-x-2">
            <div class="self-end shrink-0">"Ethan Henry"</div>
            <small class="hidden md:block">
                "Opinions and statements are my own and not representative of any
                 past/present or future employers"
            </small>
            <div class="self-end shrink-0">"© 2025"</div>
        </footer>
    }
}
