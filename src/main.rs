#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use]
extern crate lazy_static;

extern crate rocket;

extern crate horaire;

extern crate sms_freemobile_api;

//use std::io;
use std::path::{Path, PathBuf};

use rocket::response::content;
use rocket::response::NamedFile;
//use rocket::response::status;
use rocket::request::LenientForm;

use horaire::source::ratp::ratp;
use horaire::source::sncf::sncf;
use horaire::source::transilien::transilien;
use horaire::timelines::get_time_lines_html;

use sms_freemobile_api::sms_service::SmsService;

lazy_static! {
    static ref SMS_SERVICE: SmsService = { SmsService::new("Accounts.toml") };
}

fn liste_url() -> String {
    let mut s = String::new();
    s.push_str("<h1>Transilien</h1>");
    let stations = [
        ("Chars", "CHR"),
        ("Conflans-Ste-Honorine", "CSH"),
        ("Pontoise", "PSE"),
        ("Sartrouville", "SVL"),
        ("Cergy-Préfecture", "CYP"),
        ("Paris-Saint-Lazare", "PSL"),
        ("Nanterre-Université", "NUN"),
    ];
    for station in stations.iter() {
        s.push_str(
            format!(
                "<a href=\"/transilien/{}\" >{}</a><p>",
                station.1, station.0
            ).as_str(),
        );
    }

    s.push_str("<h1>RATP</h1>");
    let stations = [
        ("Auber", "A", "Auber"),
        ("Le Vesinet le Pecq", "A", "Le Vesinet le Pecq"),
    ];
    for station in stations.iter() {
        s.push_str(
            format!(
                "<a href=\"/ratp/{}/{}\" >{}</a><p>",
                station.1, station.2, station.0
            ).as_str(),
        );
    }

    s.push_str("<h1>SNCF DEPART</h1>");
    let stations = [("Paris-Saint-Lazare", "PSL")];
    for station in stations.iter() {
        s.push_str(format!("<a href=\"/sncf/dest/{}\" >{}</a><p>", station.1, station.0).as_str());
    }
    s.push_str("<h1>SNCF ARRIVEE</h1>");
    for station in stations.iter() {
        s.push_str(
            format!(
                "<a href=\"/sncf/arriv/{}\" >{}</a><p>",
                station.1, station.0
            ).as_str(),
        );
    }

    s
}

#[get("/transilien/<station>", format = "text/html")]
fn rt_transilien(station: String) -> content::Html<String> {
    content::Html(get_time_lines_html(
        transilien(station.as_str()).unwrap().iter(),
    ))
}

#[get("/ratp/<line>/<station>", format = "text/html")]
fn rt_ratp(line: String, station: String) -> content::Html<String> {
    content::Html(get_time_lines_html(
        ratp(line.as_str(), station.as_str()).unwrap().iter(),
    ))
}
#[get("/sncf/dest/<station>", format = "text/html")]
fn rt_sncf_dest(station: String) -> content::Html<String> {
    content::Html(get_time_lines_html(
        sncf(station.as_str(), true).unwrap().iter(),
    ))
}
#[get("/sncf/arriv/<station>", format = "text/html")]
fn rt_sncf_arriv(station: String) -> content::Html<String> {
    content::Html(get_time_lines_html(
        sncf(station.as_str(), false).unwrap().iter(),
    ))
}

#[get("/", format = "text/html")]
fn rt_index() -> content::Html<String> {
    content::Html(liste_url())
}

#[catch(404)]
fn rt_404(_req: &rocket::Request) -> content::Html<String> {
    content::Html(liste_url())
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[derive(FromForm)]
struct Sms {
    user: String,
    msg: String,
}

#[post("/writeMsg", data = "<sms>")]
fn write_msg(sms: LenientForm<Sms>) -> content::Html<String> {
    match SMS_SERVICE.sms_user(sms.get().user.as_str(), sms.get().msg.as_str()) {
        Ok(msg) | Err(msg) => content::Html(msg),
    }
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                rt_index,
                rt_transilien,
                rt_ratp,
                rt_sncf_dest,
                rt_sncf_arriv,
                write_msg,
                files
            ],
        )
        .catch(catchers![rt_404])
        .launch();
}
