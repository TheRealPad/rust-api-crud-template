use crate::api_template::api_template;

mod constants;
mod routes;
mod api_template;
mod middlewares;
mod controllers;
mod models;
mod libs;

#[macro_use]
extern crate serde_derive;

fn main() {
    api_template()
}
