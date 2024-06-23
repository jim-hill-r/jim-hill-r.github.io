#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
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

pub fn main() {}
