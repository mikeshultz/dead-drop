#[macro_use]
extern crate rocket;
use either::{Either, Left, Right};
use regex::Regex;
use rocket::http::{Accept, Status};
use rocket::response::{content, Redirect, Responder};
use rocket::serde::json::json;
use rocket::serde::{json::Json, json::Value, Deserialize, Serialize};
use rocket::Request;
use rocket_dyn_templates::{context, Template};
use rocket_governor::{Method, Quota, RocketGovernable, RocketGovernor};
use std::collections::HashMap;
use std::env::var;
use std::sync::Mutex;
use std::time::SystemTime;

const VALID_NAME: &str = r"^([A-Za-z0-9_-]{1,24})$";
const DEFAULT_TITLE: &str = "dead drop";

///
/// Data structures
///

#[derive(Debug)]
struct DeadState {
    notes: Mutex<HashMap<String, Note>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde()]
struct Note {
    updated: u64,
    body: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde()]
struct NoteRequest {
    body: String,
}

#[derive(Debug, Serialize)]
#[serde()]
struct UpdatedResponse {
    updated: u64,
}

pub struct Limiter;

impl<'r> RocketGovernable<'r> for Limiter {
    fn quota(_method: Method, _route_name: &str) -> Quota {
        // 2rps
        Quota::per_second(Self::nonzero(2u32))
    }
}

#[derive(Responder)]
enum JsonResponse<T> {
    #[response(status = 200)]
    Ok(Json<T>),
    #[response(status = 404)]
    NotFound(Value),
}

/// Get safe name from given param value
fn safe_name(slug: &str) -> Option<&str> {
    let re = Regex::new(VALID_NAME).unwrap();

    let matched = re.find(slug);

    match matched {
        Some(m) => Some(m.as_str()),
        None => None,
    }
}

/// Return the page title, derived from the TITLE env var
fn title() -> String {
    var("TITLE").unwrap_or(DEFAULT_TITLE.to_string())
}

///
/// Routes
///

#[get("/static/dead-drop.js")]
fn js(_limiter: RocketGovernor<Limiter>) -> content::RawJavaScript<&'static str> {
    content::RawJavaScript(include_str!("../static/dead-drop.js"))
}

#[get("/favicon.ico")]
fn favicon() -> Status {
    Status::NotFound
}

#[get("/robots.txt")]
fn robots() -> &'static str {
    "User-agent: *\n\
Allow: /$\n\
Disallow: /\n"
}

#[get("/<name>/updated", format = "application/json")]
fn get_updated_json<'r>(
    state: &'r rocket::State<DeadState>,
    name: &str,
    _limiter: RocketGovernor<Limiter>,
) -> JsonResponse<UpdatedResponse> {
    match safe_name(name) {
        Some(safe_name) => {
            return match state.notes.lock().unwrap().get(safe_name) {
                Some(note) => JsonResponse::Ok(Json(UpdatedResponse {
                    updated: note.updated,
                })),
                None => JsonResponse::Ok(Json(UpdatedResponse { updated: 0 })),
            };
        }
        None => JsonResponse::NotFound(json!({
            "success": false,
            "code": 404,
            "error": "Note not found",
        })),
    }
}

#[post("/<name>", format = "application/json", data = "<input>")]
fn post_note<'r>(
    state: &'r rocket::State<DeadState>,
    name: &str,
    input: Json<NoteRequest>,
    _limiter: RocketGovernor<Limiter>,
) -> JsonResponse<Note> {
    match safe_name(name) {
        Some(safe_name) => {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()
                .expect("Failed to get system time")
                .as_secs();

            state.notes.lock().unwrap().insert(
                String::from(safe_name),
                Note {
                    body: input.body.as_str().to_string(),
                    updated: now,
                },
            );

            JsonResponse::Ok(Json(Note {
                body: input.body.as_str().to_string(),
                updated: now,
            }))
        }
        // 404 on a bad name
        None => JsonResponse::NotFound(json!({
            "success": false,
            "code": 400,
            "error": "Invalid name",
        })),
    }
}

#[get("/<name>", format = "application/json", rank = 2)]
fn get_note_json<'r>(
    state: &'r rocket::State<DeadState>,
    name: &str,
    _limiter: RocketGovernor<Limiter>,
) -> JsonResponse<Note> {
    match safe_name(name) {
        Some(safe_name) => {
            return match state.notes.lock().unwrap().get(safe_name) {
                Some(note) => JsonResponse::Ok(Json(note.clone())),
                None => JsonResponse::Ok(Json(Note {
                    body: String::new(),
                    updated: 0,
                })),
            };
        }
        None => JsonResponse::NotFound(json!({
            "success": false,
            "code": 404,
            "error": "Not found",
        })),
    }
}

#[get("/<name>", format = "text/html")]
fn get_note<'r>(
    state: &'r rocket::State<DeadState>,
    name: &str,
    _limiter: RocketGovernor<Limiter>,
) -> Either<Template, Redirect> {
    match safe_name(name) {
        Some(safe_name) => {
            return match state.notes.lock().unwrap().get(safe_name) {
                Some(note) => Left(Template::render(
                    "notepad",
                    context! {
                        content_title: format!("{} ({})", title(), safe_name),
                        note_body: note.body.as_str(),
                    },
                )),
                None => Left(Template::render(
                    "notepad",
                    context! {
                        content_title: format!("{} ({})", title(), safe_name),
                    },
                )),
            };
        }
        // 301 redirect to /
        None => Right(Redirect::moved("/")),
    }
}

#[get("/")]
fn index(_limiter: RocketGovernor<Limiter>) -> Template {
    Template::render(
        "index",
        context! {
            content_title: title(),
        },
    )
}

///
/// Handlers
///

fn response_error_html(code: u16, message: &str) -> Template {
    Template::render(
        "error",
        context! {
            content_title: title(),
            error_description: format!("Error {} - {}", code, message),
        },
    )
}

fn response_error_json(code: u16, message: &str) -> String {
    format!(
        "{{\"success\": false, \"code\": {}, \"error\": \"{}\"}}",
        code, message
    )
}

/// Catch and handle the error statuses
#[catch(default)]
fn default_catcher(status: Status, request: &Request) -> Either<Template, String> {
    let reason = status.reason();

    if request.accept().is_some_and(|a| *a == Accept::JSON) {
        return Right(response_error_json(
            status.code,
            match reason {
                Some(reason) => reason,
                None => "unknown",
            },
        ));
    }

    Left(response_error_html(
        status.code,
        match reason {
            Some(reason) => reason,
            None => "unknown",
        },
    ))
}

///
/// Main
///

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                index,
                js,
                favicon,
                get_note,
                get_note_json,
                get_updated_json,
                post_note,
                robots,
            ],
        )
        .register("/", catchers![default_catcher])
        .manage(DeadState {
            notes: Mutex::new(HashMap::new()),
        })
        .attach(Template::fairing())
}
