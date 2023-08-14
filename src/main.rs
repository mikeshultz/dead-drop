#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::response::content;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Request, State};
use rocket_dyn_templates::{context, Template};

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::SystemTime;

///
/// Data structures
///

#[derive(Debug, Deserialize, Serialize)]
#[serde()]
struct Note {
    updated: AtomicU64,
    body: Mutex<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde()]
struct NoteRequest {
    body: Mutex<String>,
}

#[derive(Debug, Serialize)]
#[serde()]
struct UpdatedResponse {
    updated: u64,
}

///
/// Routes
///

#[get("/")]
fn index() -> Template {
    Template::render(
        "notepad",
        context! {
            title: "dead drop",
        },
    )
}

#[get("/dead-drop.js")]
fn js() -> content::RawJavaScript<&'static str> {
    content::RawJavaScript(include_str!("../static/dead-drop.js"))
}

#[get("/notepad", format = "json")]
fn get_notepad(note: &State<Note>) -> Json<&Note> {
    Json(note.inner())
}

#[get("/updated", format = "json")]
fn get_updated(note: &State<Note>) -> Json<UpdatedResponse> {
    Json(UpdatedResponse {
        updated: note.inner().updated.load(Ordering::Relaxed),
    })
}

#[post("/notepad", format = "json", data = "<input>")]
fn post_notepad(state: &State<Note>, input: Json<NoteRequest>) -> Json<&Note> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .expect("Failed to get system time")
        .as_secs();

    // Update cache
    state.updated.store(now, Ordering::Relaxed);
    *state.body.lock().unwrap() = String::from(input.body.lock().unwrap().clone());

    Json(state.inner())
}

///
/// Handlers
///

fn response_error(code: u16, message: &str) -> String {
    format!(
        "{{\"success\": false, \"code\": {}, \"error\": \"{}\"}}",
        code, message
    )
}

/// Catch the error statuses and respond with JSON
#[catch(default)]
fn default_catcher(status: Status, request: &Request) -> String {
    println!("{}: {}", status, request.uri());

    let reason = status.reason();
    response_error(
        status.code,
        match reason {
            Some(reason) => reason,
            None => "unknown",
        },
    )
}

///
/// Main
///

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![index, js, get_notepad, get_updated, post_notepad],
        )
        .register("/", catchers![default_catcher])
        .manage(Note {
            body: Mutex::new(String::new()),
            updated: AtomicU64::new(0),
        })
        .attach(Template::fairing())
}
