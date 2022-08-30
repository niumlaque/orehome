use actix_web::{web, FromRequest, Handler, Responder};
use std::fmt;

#[derive(Debug)]
pub enum Method {
    Get,
    #[allow(dead_code)]
    Post,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
        }
    }
}

pub struct Route<'a> {
    pub(crate) path: &'a str,
    pub(crate) route: actix_web::Route,
    #[allow(dead_code)]
    pub(crate) method: Method,
}

impl<'a> Route<'a> {
    pub fn get<F, Args>(path: &'a str, f: F) -> Self
    where
        F: Handler<Args>,
        Args: FromRequest + 'static,
        F::Output: Responder + 'static,
    {
        Self {
            path,
            route: web::get().to(f),
            method: Method::Get,
        }
    }
}
