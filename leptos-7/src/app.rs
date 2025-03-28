use leptos::{component, view, IntoView};
use leptos_meta::provide_meta_context;
use leptos_router::{
    components::{Route, Router, Routes},
    path, SsrMode, StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    #[cfg(feature = "ssr")]
    {
        use leptos::prelude::*;
        use leptos_meta::{Body, Html, Link, Meta, Script, Stylesheet, Title};

        use crate::components::{
            content::blog_view::BlogView,
            layout::{site_footer::SiteFooter, site_header::SiteHeader},
        };

        use crate::routes::{
            about::About, blog::Blog, contact::Contact, home::HomePage, links::SocialLinks,
            not_found::NotFound, projects::Projects,
        };
        return view! {
            <Html {..} lang="en" />
            <Meta charset="UTF-8" />
            <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <Meta name="author" content="Ethan (Tadgh) Henry" />
            <Meta name="description" content="Welcome, stay awhile and learn something :)" />
            <Title text="Home" formatter=|text| format!("TEH: {text}") />
            <Stylesheet id="leptos" href="/pkg/tadgh-blog-leptos.css" />

            <Link
                rel="preload"
                href="/fonts/nunl7.woff2"
                as_="font"
                type_="font/woff2"
                crossorigin=""
                fetchpriority="high"
            />

            <Link
                rel="preload"
                href="/fonts/nunlr.woff2"
                as_="font"
                type_="font/woff2"
                crossorigin=""
                fetchpriority="high"
            />

            <Link rel="icon" href="/favicon.ico" />
            <Link rel="shortcut icon" href="/favicon.ico" />
            <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
            <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
            <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
            <Link rel="manifest" href="/site.webmanifest" />
            <Script
                // attrs=vec![("data-domain", Attribute::String(leptos::Oco::Borrowed("tadgh.dev")))]
                defer=""
                src="https://tadgh.dev/js/script.js"
            ></Script>
            <Script

                type_="module"
                defer=""
                src="/app.js"
            ></Script>
            <Body {..} class="tail" />
            <Router>
                <SiteHeader />
                <main class="palms">
                    <Routes fallback=move || "Not found.">
                        <Route path=StaticSegment("/") view=HomePage />
                        <Route path=StaticSegment("/about") view=About />
                        <Route path=StaticSegment("/projects") view=Projects />
                        <Route path=StaticSegment("/blog") view=Blog />
                        <Route path=path!("/blog/:id") view=BlogView ssr=SsrMode::Async />
                        <Route path=StaticSegment("/contact") view=Contact />
                        <Route path=StaticSegment("/links") view=SocialLinks />
                        <Route path=path!("/*any") view=NotFound />
                    </Routes>
                </main>
                <SiteFooter />
            </Router>
        };
    }
}
