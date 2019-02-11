#![feature(proc_macro_hygiene, decl_macro)]

use std::path::{Path, PathBuf};

use rocket::{
    request::LenientForm,
    response::{content, NamedFile},
    *,
};

use horaire::{
    source::{ratp::ratp, sncf::sncf, transilien::transilien},
    timelines::get_time_lines_html,
};
use lazy_static::*;

use sms_freemobile_api::sms_service::SmsService;

lazy_static! {
    static ref SMS_SERVICE: SmsService = { SmsService::new("Accounts.toml") };
}

fn liste_url() -> String {
    let mut s = String::new();
    s.push_str("<h1>Transilien</h1>");
    let stations = [
        ("Chars", "CHARS", 8738119),
        ("Conflans-Ste-Honorine", "CONFLANS SAINTE-HONORINE", 8738189),
        ("Pontoise", "PONTOISE", 8727613),
        ("Sartrouville", "SARTROUVILLE", 8738641),
        ("Cergy-Préfecture", "CERGY PREFECTURE", 8738190),
        ("Paris-Saint-Lazare", "GARE DE PARIS SAINT-LAZARE", 8738400),
        ("Nanterre-Université", "NANTERRE UNIVERSITE", 8738631),
    ];
    for station in stations.iter() {
        s.push_str(
            format!(
                "<a href=\"/transilien/{}/{}\" >{}</a><p>",
                station.2, station.1, station.0
            )
            .as_str(),
        );
    }

    s.push_str("<h1>RATP</h1>");
    let stations = [
        ("Auber", "A", "Auber"),
        ("Le Vesinet le Pecq", "A", "Le Vesinet-Le Pecq"),
    ];
    for station in stations.iter() {
        s.push_str(
            format!(
                "<a href=\"/ratp/{}/{}\" >{}</a><p>",
                station.1, station.2, station.0
            )
            .as_str(),
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
            )
            .as_str(),
        );
    }

    s
}

#[get("/transilien/<uic>/<station>", format = "text/html")]
fn rt_transilien(uic: u32, station: String) -> content::Html<String> {
    content::Html(get_time_lines_html(
        transilien(station.as_str(), uic).unwrap().iter(),
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
    match SMS_SERVICE.sms_user(sms.user.as_str(), sms.msg.as_str()) {
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
        .register(catchers![rt_404])
        .launch();
}
