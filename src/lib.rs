use std::collections::HashMap;

use anyhow::Result;
use handlebars::Handlebars;
use pulldown_cmark as markdown;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};
use types::Feed;

mod redis;
mod types;

const FINGER_PATH: &str = "/files/finger.json";
const FRIENDS_PATH: &str = "/files/friends.json";
const TEMPLATES_DIR: &str = "/files/templates";

/// A simple Spin HTTP component.
#[http_component]
fn finger(req: Request) -> Result<Response> {
    let mut hbs = Handlebars::new();
    hbs.register_templates_directory(".hbs", TEMPLATES_DIR)?;
    match req.uri().path() {
        "/" => do_index(hbs),
        "/plan" => do_plan(hbs),
        "/plan/edit" => do_plan_edit(req, hbs),
        "/finger" => do_finger(hbs),
        "/feed" => do_feed(hbs),
        "/test-redis" => test_redis(),
        "/uc" => do_uc(),
        _ => Ok(http::Response::builder()
            .header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
            .status(400)
            .body(Some("Not Found".into()))?),
    }
}

// Given an HTML body, return an HTTP 200 response
fn html_ok(msg: String) -> Result<Response> {
    Ok(http::Response::builder()
        .header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
        .status(200)
        .body(Some(msg.into()))?)
}

fn finger_plan() -> Result<types::FingerPlan> {
    let finger = read_finger()?;
    let uname = &finger.username.clone();
    Ok(types::FingerPlan {
        finger,
        plan: md_to_html(&redis::read_plan(&uname)?),
        friends: read_friends()?,
        self_link: None,
    })
}

// Generate the index.
fn do_index(hbs: Handlebars) -> Result<Response> {
    let data = finger_plan()?;
    let out = hbs.render("index", &data)?;

    html_ok(out)
}

fn do_uc() -> Result<Response> {
    let finger = read_finger()?;
    let uname = finger.username.clone();
    let uc = types::FingerPlan {
        finger,
        plan: redis::read_plan(&uname)?, // The raw version, not HTML
        friends: read_friends()?,
        self_link: None,
    };
    let out = serde_json::to_string(&uc)?;
    Ok(http::Response::builder()
        .header(
            http::header::CONTENT_TYPE,
            "application/json; charset=utf-8",
        )
        .status(200)
        .body(Some(out.into()))?)
}

/// Generate the finger page
fn do_finger(hbs: Handlebars) -> Result<Response> {
    let finger_json = read_finger()?;
    let mut data = HashMap::new();
    data.insert("finger", finger_json);

    let msg = hbs.render("finger", &data)?;
    html_ok(msg)
}
/// Generate the plan page
fn do_plan(hbs: Handlebars) -> Result<Response> {
    let data = finger_plan()?;
    let msg = hbs.render("plan", &data)?;
    html_ok(msg)
}

fn do_plan_edit(req: Request, hbs: Handlebars) -> Result<Response> {
    // TODO: Add HTTP basic auth or something better

    let finger = read_finger()?;
    let plan = redis::read_plan(&finger.username)?;
    let friends = read_friends()?;

    match req.method() {
        &http::method::Method::POST => {
            match req.body() {
                Some(body) => {
                    let form: types::EditPlanQueryParams = serde_qs::from_bytes(&body.to_vec())?;
                    redis::write_plan(&finger.username, form.plan)?;
                }
                None => {
                    // I guess we do nothing?
                    redis::write_plan(&finger.username, "Placeholder text".to_owned())?;
                }
            }
            // Update Redis and then redirect to /plan
            Ok(http::Response::builder()
                .header(http::header::LOCATION, "/plan")
                .status(303)
                .body(None)?)
        }
        _ => {
            let data = types::FingerPlan {
                finger,
                plan,
                friends,
                self_link: None,
            };
            // Display the editor.
            let msg = hbs.render("edit_plan", &data)?;
            html_ok(msg)
        }
    }
}

/// Read all of friends' feeds and display
fn do_feed(hbs: Handlebars) -> Result<Response> {
    //let friends = read_friends_and_load()?;
    let data = finger_plan()?;

    let mut friends_plans = vec![data.clone()];
    for friend in &data.friends {
        friends_plans.push(friend.load_finger()?.clone());
    }

    let feed = Feed {
        finger: data.finger,
        friends_plans: friends_plans,
    };

    let msg = hbs.render("feed", &feed)?;
    html_ok(msg)
}

fn read_finger() -> Result<types::Finger> {
    let finger_text = std::fs::read_to_string(FINGER_PATH)?;
    let finger_data: types::Finger = serde_json::from_str(&finger_text)?;

    Ok(finger_data)
}

/// Loads the friends file
fn read_friends() -> Result<Vec<types::Friend>> {
    let friends_text = std::fs::read_to_string(FRIENDS_PATH)?;
    let friends_list: Vec<types::Friend> = serde_json::from_str(&friends_text)?;
    Ok(friends_list)
}

/// Convert markdown to HTML.
fn md_to_html(input: &str) -> String {
    // Turn on all the markdown options
    let opts = markdown::Options::all();
    let parser = markdown::Parser::new_ext(input, opts);
    let mut output = String::new();
    markdown::html::push_html(&mut output, parser);

    output
}

fn test_redis() -> Result<Response> {
    redis::set_string("test-insert".to_owned(), "Hello".to_owned())?;
    let msg = redis::get_string("test-insert".to_owned())?;
    Ok(http::Response::builder()
        .header(http::header::CONTENT_TYPE, "text/html; charset=utf-8")
        .status(200)
        .body(Some(msg.into()))?)
}
