use actix_web::dev::ConnectionInfo;
use actix_web::{web, HttpResponse, Responder};
use tera::Tera;

pub async fn search(conn: ConnectionInfo, tmpl: web::Data<Tera>) -> impl Responder {
    tracing::info!(
        "host: {}, peer_addr: {:?}, realip_remote_addr: {:?}, scheme: {}",
        conn.host(),
        conn.peer_addr(),
        conn.realip_remote_addr(),
        conn.scheme()
    );
    let context = tera::Context::new();
    match tmpl.render("search.html", &context) {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body("")
        }
    }
}
