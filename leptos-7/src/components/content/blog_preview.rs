use crate::server_functions::blog::BlogPreview;
use leptos::{component, prelude::*, view, IntoView};

#[component]
pub fn BlogPreview(blog: BlogPreview) -> impl IntoView {
    let title = blog.get_title().to_owned();
    let desc = blog.get_description().to_owned();
    let url = format!("/blog/{}", blog.get_file_name_clean());
    view! {
        <article class="blog-pview-cont">
            <div>
                {blog
                    .get_image()
                    .map(move |image_path| {
                        view! { <img src=image_path class="blog-thumbnail-main" /> }.into_view()
                    })}
                <time datetime=blog
                    .get_date()
                    .to_string()
                    .to_owned()>{blog.get_date().to_string().to_owned()}</time> <header>
                    <h3>{title}</h3>
                </header> <p>{desc}</p> <div class="imply-other-site">
                    <a href=url>"Read"</a>
                </div>
            </div>
            {blog
                .get_image()
                .map(move |image_path| {
                    view! { <img src=image_path class="wide-thumbnail" /> }.into_view()
                })}

        </article>
    }
}
