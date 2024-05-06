#[cfg(all(
    feature = "mpa",
    not(feature = "spa"),
    not(feature = "ssh"),
    not(feature = "ssr")
))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use jimhillr_github_io::app::App;
    use leptos::*;
    use leptos_actix::{generate_route_list_with_ssg, LeptosRoutes};
    use leptos_router::build_static_routes;

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let (routes, static_data_map) = generate_route_list_with_ssg(App);

    build_static_routes(&leptos_options, App, &routes, &static_data_map)
        .await
        .unwrap();

    Ok(())
}

#[cfg(all(
    feature = "spa",
    not(feature = "mpa"),
    not(feature = "ssh"),
    not(feature = "ssr")
))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    use jimhillr_github_io::app::App;
    use leptos::{logging, mount_to_body};
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("spa mode - mounting to body");

    leptos::mount_to_body(App);
}

#[cfg(all(
    feature = "ssh",
    not(feature = "mpa"),
    not(feature = "spa"),
    not(feature = "ssr")
))]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    use jimhillr_github_io::app::App;
    use leptos::{logging, mount_to_body};
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    logging::log!("hydrate mode - hydrating");

    mount_to_body(App);
}

#[cfg(all(
    feature = "ssr",
    not(feature = "mpa"),
    not(feature = "spa"),
    not(feature = "ssh")
))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use jimhillr_github_io::app::App;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(all(
    feature = "ssr",
    not(feature = "mpa"),
    not(feature = "spa"),
    not(feature = "ssh")
))]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "mpa", feature = "spa", feature = "ssh")))]
fn main() {}
