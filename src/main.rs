#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]
extern crate rocket;

extern crate horaire;

use rocket::response::content;

use horaire::timelines::{TimeLine, get_time_lines_html};
use horaire::source::transilien::transilien;
use horaire::source::sncf::sncf;
use horaire::source::ratp::ratp;
// use horaire::errors::*;

#[get("/transilien/<station>", format = "text/html")]
fn rt_transilien(station: String) -> content::Html<String> {
    content::Html(get_time_lines_html(transilien(station.as_str()).unwrap().iter()))
}
#[get("/ratp/<line>/<station>", format = "text/html")]
fn rt_ratp(line:String, station: String) -> content::Html<String> {
    content::Html(get_time_lines_html(ratp(line.as_str(), station.as_str()).unwrap().iter()))
}
#[get("/sncf/dest/<station>", format = "text/html")]
fn rt_sncf_dest(station: String) -> content::Html<String> {
    content::Html(get_time_lines_html(sncf(station.as_str(), true).unwrap().iter()))
}
#[get("/sncf/arriv/<station>", format = "text/html")]
fn rt_sncf_arriv(station: String) -> content::Html<String> {
    content::Html(get_time_lines_html(sncf(station.as_str(), false).unwrap().iter()))
}
fn main() {
    rocket::ignite().mount("/", routes![rt_transilien, rt_ratp, rt_sncf_dest, rt_sncf_arriv]).launch();
}
