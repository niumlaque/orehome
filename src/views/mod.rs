mod list;
mod route;
mod search;

use actix_files::Files;
use actix_web::{web, App};
use route::Route;
use tera::Tera;

fn get<'a>() -> Vec<Route<'a>> {
    vec![Route::get("/", search::search)]
}

pub fn register<T>(mut app: App<T>) -> App<T>
where
    T: actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Error = actix_web::Error,
        Config = (),
        InitError = (),
    >,
{
    let templates = Tera::new("templates/**/*").unwrap();
    app = app.app_data(web::Data::new(templates));
    app = app.service(Files::new("/static", "./assets").show_files_listing());
    let routes = get();
    app = list::register(app, routes.iter());
    for item in routes.into_iter() {
        app = app.route(item.path, item.route);
        println!("[{:<4}] {}", item.method.to_string(), item.path);
    }

    println!("[{:<4}] /list", "GET");

    app
}
