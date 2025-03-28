pub mod app;
pub mod components;
pub mod routes;
pub mod server_functions;
#[cfg(feature = "ssr")]
pub mod service_handlers;
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    // mount_to_body(App);
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
