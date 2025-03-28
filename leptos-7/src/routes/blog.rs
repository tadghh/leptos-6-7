use leptos::{component, prelude::*, IntoView};
#[component]
pub fn Blog() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use leptos::*;

        use crate::components::error_notif::ErrorNotification;

        use crate::{
            components::content::blog_preview::{BlogPreview, BlogPreviewProps},
            server_functions::content_io::get_blog_previews_cache,
        };

        use leptos_meta::Title;
        let blog_previews =
            Resource::new(|| (), |_| async move { get_blog_previews_cache().await });

        return view! {
            <Title text="Posts" />
            <div class="projects-container">
                <h1 class="page-title">"Blog posts"</h1>
                <div class="flex flex-col items-center space-y-5">
                    <Transition fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>
                        {move || {
                            blog_previews
                                .get()
                                .map(|previews| match previews {
                                    Err(e) => {
                                        view! { <ErrorNotification error_text=e.to_string() /> }
                                            .into_any()
                                    }
                                    Ok(previews) => {
                                        if previews.is_empty() {
                                            view! {
                                                <ErrorNotification error_text="No previews found"
                                                    .to_string() />
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <ul class="flex flex-col gap-y-5 w-full">

                                                    {previews
                                                        .into_iter()
                                                        .map(|blog_preview| BlogPreview(BlogPreviewProps {
                                                            blog: blog_preview,
                                                        }))
                                                        .collect_view()}
                                                </ul>
                                            }
                                                .into_any()
                                        }
                                    }
                                })
                                .unwrap_or_else(|| {
                                    view! {
                                        <ErrorNotification error_text="Failed to load previews"
                                            .to_string() />
                                    }
                                        .into_any()
                                })
                        }}
                    </Transition>
                </div>
            </div>
        };
    }
}
