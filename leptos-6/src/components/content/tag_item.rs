use crate::server_functions::tags::TagWithContent;
use leptos::{component, view, CollectView, IntoView};

#[component]
pub fn TagItem(tag: TagWithContent) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        return view! {
            <div class="tag-container group">
                <div>
                    <div>{&tag.tag_name}</div>

                    <ul class="tag-list">
                        {tag
                            .get_blogs()
                            .iter()
                            .map(|blog| {
                                view! {
                                    <li>
                                        <a href=format!(
                                            "/blog/{}",
                                            blog.get_file_name_clean(),
                                        )>{blog.get_name()}</a>
                                    </li>
                                }
                            })
                            .collect_view()}
                        {tag
                            .get_projects()
                            .iter()
                            .map(|project| {
                                view! {
                                    <li>
                                        <a
                                            href=project.get_url()
                                            target="_blank"
                                            rel="noopener noreferrer"
                                        >
                                            {project.get_name()}
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>

                </div>
            </div>
        };
    }
}
