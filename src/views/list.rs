use super::route::Route;
use actix_web::{web, App, HttpResponse, Responder};

pub fn register<'a, T, TRoutes>(app: App<T>, routes: TRoutes) -> App<T>
where
    T: actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Error = actix_web::Error,
        Config = (),
        InitError = (),
    >,
    TRoutes: std::iter::Iterator<Item = &'a Route<'a>>,
{
    let html = create_html(routes);
    let app = app.app_data(web::Data::new(Index::new(html)));
    app.route("/list", web::get().to(index))
}

async fn index(html: web::Data<Index>) -> impl Responder {
    let s: String = html.get().to_string();
    HttpResponse::Ok().body(s)
}

fn create_html<'a, T: std::iter::Iterator<Item = &'a Route<'a>>>(routes: T) -> String {
    let mut ret = Vec::with_capacity(16);
    for route in routes {
        ret.push(format!("<a href={}>{}</a>", route.path, route.path));
    }
    ret.join("<br>")
}

struct Index {
    html: String,
}

impl Index {
    pub fn new(html: impl Into<String>) -> Self {
        Self { html: html.into() }
    }

    pub fn get(&self) -> &str {
        &self.html
    }
}
