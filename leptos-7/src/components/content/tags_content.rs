use crate::components::{self, content::tag_item::TagItem};
use leptos::{component, prelude::*, IntoView};
#[component]
pub fn Tags() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use crate::{
            components::error_notif::ErrorNotification,
            server_functions::tags::get_categories_with_content,
        };
        use leptos::*;

        let categories = Resource::new(
            || (),
            |_| async move { get_categories_with_content().await },
        );
        view! {
            <Transition fallback=move || {
                view! { <p class="text-gray-600">"Loading..."</p> }
            }>
                {move || {
                    view! {
                        {move || {
                            categories
                                .get()
                                .map(|result| match result {
                                    Err(e) => {
                                        view! { <ErrorNotification error_text=e.to_string() /> }
                                            .into_any()
                                    }
                                    Ok(categories) => {
                                        view! {
                                            <div class="tag-content">
                                                {categories
                                                    .iter()
                                                    .map(|category| {
                                                        view! {
                                                            <div>
                                                                <h3>{category.category_name.clone()}</h3>
                                                                {category
                                                                    .get_tags()
                                                                    .iter()
                                                                    .map(|tag| {
                                                                        TagItem(components::content::tag_item::TagItemProps {
                                                                            tag: tag.clone(),
                                                                        })
                                                                    })
                                                                    .collect_view()}
                                                            </div>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        }
                                            .into_any()
                                    }
                                })
                        }}
                    }
                }}
            </Transition>
        }
    }
}
