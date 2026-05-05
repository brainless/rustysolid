use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use shared_types::HeartbeatResponse;

mod auth;
mod config;

#[get("/api/heartbeat")]
async fn heartbeat() -> impl Responder {
    let payload = HeartbeatResponse {
        status: "ok".to_string(),
        service: env!("CARGO_PKG_NAME").to_string(),
    };

    HttpResponse::Ok().json(payload)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::load()
        .unwrap_or_else(|e| { eprintln!("error: {e}"); std::process::exit(1) });

    println!(
        "Backend listening on http://{}:{}",
        config.server.host, config.server.port
    );

    let gui_origin_ip = format!("http://127.0.0.1:{}", config.gui.port);
    let gui_origin_local = format!("http://localhost:{}", config.gui.port);
    let admin_origin_ip = format!("http://127.0.0.1:{}", config.admin_gui.port);
    let admin_origin_local = format!("http://localhost:{}", config.admin_gui.port);
    let domain_origin_https = config
        .deploy
        .as_ref()
        .map(|d| format!("https://{}", d.domain_name));
    let domain_origin_http = config
        .deploy
        .as_ref()
        .map(|d| format!("http://{}", d.domain_name));

    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allowed_origin(&gui_origin_ip)
            .allowed_origin(&gui_origin_local)
            .allowed_origin(&admin_origin_ip)
            .allowed_origin(&admin_origin_local);

        if let Some(ref origin) = domain_origin_https {
            cors = cors.allowed_origin(origin);
        }
        if let Some(ref origin) = domain_origin_http {
            cors = cors.allowed_origin(origin);
        }

        let cors = cors.allowed_methods(vec!["GET"]).allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(web::JsonConfig::default())
            .service(heartbeat)
    })
    .bind((config.server.host.as_str(), config.server.port))?
    .run()
    .await
}
