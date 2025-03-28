use crate::server_functions::blog::BlogPreview;
use leptos::{component, view, IntoView};

#[component]
pub fn BlogPreview(blog_preview: BlogPreview) -> impl IntoView {
    view! {
        <article class="blog-pview-cont">
            <div>
                {if let Some(image_path) = blog_preview.get_image() {
                    view! {
                        <img
                            src=image_path
                            class="blog-thumbnail-main"
                            alt=blog_preview.get_title()
                        />
                    }
                        .into_view()
                } else {
                    view! { <></> }.into_view()
                }}
                <time date=blog_preview
                    .get_date()
                    .to_string()>{blog_preview.get_date().to_string()}</time> <header>
                    <h3>{blog_preview.get_title()}</h3>
                </header> <p>{blog_preview.get_description()}</p> <div class="imply-other-site">
                    <a href=format!("/blog/{}", blog_preview.get_file_name_clean())>"Read"</a>
                </div>
            </div>
            {if let Some(image_path) = blog_preview.get_image() {
                view! { <img src=image_path class="wide-thumbnail" alt=blog_preview.get_title() /> }
                    .into_view()
            } else {
                view! { <></> }.into_view()
            }}

        </article>
    }
}
