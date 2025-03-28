use crate::server_functions::project::ProjectPreview;
use leptos::{component, view, IntoView};

#[component]
pub fn ProjectPreview(project_preview: ProjectPreview) -> impl IntoView {
    view! {
        <a href=project_preview.create_href() class="flex flex-col cursor-default prj-item">
            <article class="prj-contain">
                <header>
                    <h3 class="prj-title">{project_preview.get_title()}</h3>
                </header>
                <p class="prj-description">{project_preview.get_description()}</p>
                // <section id="prj-cat"></section>
                <time date=project_preview.get_date().to_string() class="prj-date">
                    {project_preview.get_date().to_string()}
                </time>
            </article>
            <div class="pr-1.5 pb-1 ml-auto text-lg font-bold underline cursor-pointer">
                "View Project"
            </div>
        </a>
    }
}
