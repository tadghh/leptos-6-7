use actix_files::NamedFile;
use actix_web::{http::header::ContentType, Responder, Result};

use std::path::PathBuf;

// Get the base directory based on compile-time configuration
#[cfg(blog_test)]
pub fn get_base_dir() -> &'static str {
    "./public"
}

#[cfg(not(blog_test))]
pub fn get_base_dir() -> &'static str {
    "./site"
}

// Structure to hold file handler configuration
#[derive(Clone)]
pub struct StaticFileConfig {
    pub path: &'static str,
    pub content_type: ContentType,
}

// Helper function to handle static files
pub async fn handle_static_file(config: &StaticFileConfig) -> Result<impl Responder> {
    let base_path = get_base_dir();
    let full_path = PathBuf::from(base_path).join(&config.path[1..]); // Remove leading slash

    Ok(NamedFile::open_async(full_path)
        .await?
        .customize()
        .insert_header(config.content_type.clone()))
}

// Define static file configurations
pub const STATIC_FILES: &[(&str, &str)] = &[
    ("/robots.txt", "text/plain"),
    ("/sitemap.xml", "application/xml"),
    ("/favicon.ico", "image/x-icon"),
    ("/site.webmanifest", "application/manifest+json"),
    ("/safari-pinned-tab.svg", "image/svg+xml"),
    ("/android-chrome-192x192.png", "image/png"),
    ("/android-chrome-256x256.png", "image/png"),
    ("/rainbow.js", "text/javascript"),
    ("/app.js", "text/javascript"),
    ("/apple-touch-icon.png", "image/png"),
    ("/favicon-16x16.png", "image/png"),
    ("/favicon-32x32.png", "image/png"),
];

pub const FONT_FILES: &[(&str, &str)] = &[
    ("/fonts/CmtMnCB.woff2", "font/woff2"),
    ("/fonts/nunl3.woff2", "font/woff2"),
    ("/fonts/nunl5.woff2", "font/woff2"),
    ("/fonts/nunl6.woff2", "font/woff2"),
    ("/fonts/nunl7.woff2", "font/woff2"),
    ("/fonts/nunlr2.woff2", "font/woff2"),
];
