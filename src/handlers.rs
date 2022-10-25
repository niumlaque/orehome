use actix_web::{web, HttpResponse, Responder};
use tera::Tera;

pub async fn search(tmpl: web::Data<Tera>) -> impl Responder {
    let context = tera::Context::new();
    match tmpl.render("search.html", &context) {
        Ok(v) => HttpResponse::Ok().body(v),
        Err(e) => {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().body("")
        }
    }
}
