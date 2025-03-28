use crate::server_functions::project::ProjectPreview;
use leptos::{component, prelude::*, view, IntoView};
#[component]
pub fn ProjectPreview(project_preview: ProjectPreview) -> impl IntoView {
    let date = project_preview.get_date().to_string().to_owned();
    view! {
        <a
            href=project_preview.create_href().to_owned()
            class="flex flex-col cursor-default prj-item"
        >
            <article class="prj-contain">
                <header>
                    <h3 class="prj-title">{project_preview.get_title().to_owned()}</h3>
                </header>
                <p class="prj-description">{project_preview.get_description().to_owned()}</p>
                // <section id="prj-cat"></section>
                <time datetime=date class="prj-date">
                    {project_preview.get_date().to_string().to_owned()}
                </time>
            </article>
            <div class="pr-1.5 pb-1 ml-auto text-lg font-bold underline cursor-pointer">
                "View Project"
            </div>
        </a>
    }
}
