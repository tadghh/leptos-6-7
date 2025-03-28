#[cfg(feature = "ssr")]
use crate::server_functions::content_io::get_prepared_blog;
use crate::{
    components::error_notif::ErrorNotification, server_functions::content_io::get_view_count_blog,
};
use leptos::{prelude::*, *};
use leptos_meta::{provide_meta_context, Meta, Title};
use leptos_router::use_params_map;
#[component]
pub fn BlogView() -> impl IntoView {
    let params = use_params_map();
    provide_meta_context();
    // its not really an ID its the normalized name. I was being lazy
    let post_query = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    let post = create_resource(
        move || post_query().clone(),
        |id| async move { get_prepared_blog(id).await },
    );

    let view_count = create_blocking_resource(
        move || post_query().clone(),
        |id| async move { get_view_count_blog(id).await },
    );

    view! {
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>

            {move || {
                post.get()
                    .map(|post_info| match post_info {
                        Ok(post_info) => {
                            match post_info {
                                Some(post) => {
                                    view! {
                                        <Title text=post.get_title().clone() />
                                        <Meta
                                            name="description"
                                            content=post.get_description().to_string()
                                        />
                                        <div class="blog-tail">
                                            <h1 class="page-title">{post.get_title()}</h1>

                                            <div class="blog-page-container">
                                                <div class="flex flex-col-reverse justify-between sm:flex-row">
                                                    <div class="blog-header-content">
                                                        <h2 class="mt-4 sm:mt-0">"Chapter Links:"</h2>
                                                        <div>
                                                            {match post.get_chapters() {
                                                                None => {
                                                                    view! { <p>"No chapters found in document."</p> }
                                                                        .into_view()
                                                                }
                                                                Some(chapters) => {
                                                                    view! {
                                                                        <ul class="underline underline-offset-2 decoration-blue-300">
                                                                            {chapters
                                                                                .into_iter()
                                                                                .map(|post| {
                                                                                    view! {
                                                                                        <li>
                                                                                            <a href=format!("#{}", post.replace(" ", "-"))>{post}</a>
                                                                                        </li>
                                                                                    }
                                                                                })
                                                                                .collect_view()
                                                                                .into_view()}
                                                                        </ul>
                                                                    }
                                                                        .into_view()
                                                                }
                                                            }}

                                                        </div>
                                                    </div>
                                                    {if let Some(image_path) = post.get_image_url() {
                                                        view! { <img class="blog-header-image" src=image_path /> }
                                                            .into_view()
                                                    } else {
                                                        view! { <></> }.into_view()
                                                    }}
                                                </div>
                                                <div class="view-date">
                                                    <span>
                                                        "Created: "
                                                        <time
                                                            class="text-sm italic font-semibold"
                                                            date=post.get_date().to_string()
                                                        >
                                                            {post.get_date().to_string()}
                                                        </time>
                                                    </span>

                                                    {move || {
                                                        view_count
                                                            .get()
                                                            .map(|view_count| match view_count {
                                                                Ok(views) => {
                                                                    view! {
                                                                        <span>
                                                                            "Views: "
                                                                            <span class="text-sm italic font-semibold">{views}</span>
                                                                        </span>
                                                                    }
                                                                        .into_view()
                                                                }
                                                                Err(views_err) => {

                                                                    view! {
                                                                        <ErrorNotification error_text=views_err.to_string() />
                                                                    }
                                                                        .into_view()
                                                                }
                                                            })
                                                    }}
                                                </div>
                                                {match post.get_content_html() {
                                                    None => {
                                                        view! {
                                                            <ErrorNotification error_text="Server: Blog file missing?"
                                                                .to_string() />
                                                        }
                                                            .into_view()
                                                    }
                                                    Some(post_content) => {
                                                        view! {
                                                            <article
                                                                class="mx-auto mt-4 blog-content-container"
                                                                inner_html=post_content
                                                            ></article>
                                                        }
                                                            .into_view()
                                                    }
                                                }}

                                            </div>
                                        </div>
                                    }
                                        .into_view()
                                }
                                None => {
                                    view! {
                                        <ErrorNotification error_text="Invalid blog ID"
                                            .to_string() />
                                    }
                                        .into_view()
                                }
                            }
                        }
                        Err(post_err) => {
                            view! { <ErrorNotification error_text=post_err.to_string() /> }
                                .into_view()
                        }
                    })
            }}
        </Transition>
    }
}
