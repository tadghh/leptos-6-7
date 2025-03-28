use leptos::{component, IntoView};

#[component]
pub fn Projects() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use leptos::*;

        use crate::{
            components::content::project_preview::{ProjectPreview, ProjectPreviewProps},
            server_functions::content_io::get_projects_cache,
        };

        use crate::components::error_notif::ErrorNotification;

        use leptos_meta::Title;
        let blog_projects = create_resource(|| (), |_| async move { get_projects_cache().await });

        return view! {
            <Title text="Projects" />
            <div class="projects-container">
                <div class="flex justify-between items-center">
                    <h1 class="page-title">"Projects"</h1>
                    <a
                        class="mt-auto mb-0.5 text-lg font-bold underline"
                        href="https://github.com/tadghh"
                    >
                        "GitHub Profile"
                    </a>
                </div>

                <span>"Projects are hosted within the tadgh.dev domain, or on GitHub."</span>
                <Transition fallback=move || {
                    view! { <p>"Loading..."</p> }
                }>
                    {move || {
                        view! {
                            {move || {
                                blog_projects
                                    .get()
                                    .map(|projects| match projects {
                                        Err(e) => {
                                            view! { <ErrorNotification error_text=e.to_string() /> }
                                                .into_view()
                                        }
                                        Ok(projects) => {
                                            if projects.is_empty() {
                                                view! { <p>"No project previews found."</p> }.into_view()
                                            } else {
                                                view! {
                                                    <div class="prj-table">
                                                        <div class="font-light">"Project"</div>
                                                        <div class="hidden md:inline-grid">"Description"</div>
                                                        <div class="text-right">"Date"</div>
                                                    </div>
                                                    {projects
                                                        .into_iter()
                                                        .map(|project| ProjectPreview(ProjectPreviewProps {
                                                            project_preview: project,
                                                        }))
                                                        .collect_view()}
                                                }
                                                    .into_view()
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }}
                        }
                    }}
                </Transition>
            </div>
        };
    }
}
