use actix_web::web;
use leptos::LeptosOptions;
use leptos_actix::{generate_route_list, handle_server_fns, LeptosRoutes};

#[allow(unreachable_code, dead_code, unexpected_cfgs)]
#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{middleware, App, HttpServer};
    use leptos::get_configuration;
    tadgh_blog_leptos::server_functions::database::init_db().await?;
    tadgh_blog_leptos::server_functions::dcache_manager::initialize_cache().await?;

    let conf = get_configuration(Some("./Cargo.toml")).await.unwrap();

    let blog_server = HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;

        #[cfg(not(blog_test))]
        {
            use actix_web_lab::middleware::RedirectHttps;
            App::new()
                .wrap(RedirectHttps::default().to_port(443))
                .wrap(middleware::Compress::default())
                .configure(|cfg| config(cfg, &leptos_options))
        }
        #[cfg(blog_test)]
        {
            App::new()
                .wrap(middleware::Compress::default())
                .configure(|cfg| config(cfg, &leptos_options))
        }
    });

    #[cfg(blog_test)]
    {
        println!("TESTING listening on http://127.0.0.1:8080");

        blog_server.bind("0.0.0.0:8080")?.run().await
    }
    #[cfg(not(blog_test))]
    {
        let conf = get_configuration(Some("./Cargo.toml")).await.unwrap();

        let addr = conf.leptos_options.site_addr;
        println!("PROD listening on https://{:?}", &addr);
        blog_server
            .bind_rustls_0_22(&addr, load_rustls_config().expect("rustls fucked"))?
            .bind("0.0.0.0:80")?
            .run()
            .await
    }
}

pub fn config(cfg: &mut web::ServiceConfig, options_lep: &LeptosOptions) {
    use actix_files::Files;
    use actix_web::web;
    use mime::Mime;
    use tadgh_blog_leptos::{app::*, service_handlers::*};
    let site_root = &options_lep.site_root;
    use actix_web::http::header::ContentType;
    // Configure all static file handlers using the new implementation
    for &(path, content_type) in STATIC_FILES.iter().chain(FONT_FILES) {
        let config = StaticFileConfig {
            path,
            content_type: ContentType(content_type.parse::<Mime>().unwrap()),
        };

        cfg.service(web::resource(path).route(web::get().to(move || {
            let config = config.clone();
            async move { handle_static_file(&config).await }
        })));
    }

    // Configure remaining non-static file services
    cfg.service(Files::new("/pkg", format!("{site_root}/pkg")))
        .route("/api/*fn_name", handle_server_fns())
        .service(Files::new("/res", "./resources"))
        .service(Files::new("/public", site_root))
        .leptos_routes(options_lep.clone(), generate_route_list(App), App)
        .app_data(web::Data::new(options_lep.to_owned()));
}

#[allow(dead_code)]
fn load_rustls_config() -> Result<rustls::ServerConfig, Box<dyn std::error::Error>> {
    // init server config builder with safe defaults
    use rustls::ServerConfig;
    use std::{fs::File, io::BufReader};

    // load TLS key/cert files
    let cert_path = std::env::var("TLS_CERT_PATH")?;
    let key_path = std::env::var("TLS_KEY_PATH")?;
    let cert_file = &mut BufReader::new(File::open(cert_path)?);
    let key_file = &mut BufReader::new(File::open(key_path)?);

    // convert files to key/cert objects
    let tls_certs = rustls_pemfile::certs(cert_file).collect::<Result<Vec<_>, _>>()?;

    let tls_key = rustls_pemfile::pkcs8_private_keys(key_file)
        .next()
        .ok_or("no tls key found in TLS_KEY_PATH")?;
    Ok(
        ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
            .with_no_client_auth()
            .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key?))?,
    )
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
