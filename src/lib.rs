use std::collections::HashMap;

use anyhow::Result;
use handlebars::Handlebars;
use pulldown_cmark as markdown;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

mod types;

const FINGER_PATH: &str = "/files/finger.json";
const PLAN_PATH: &str = "/files/plan.md";

/// A simple Spin HTTP component.
#[http_component]
fn finger(req: Request) -> Result<Response> {
    match req.uri().path() {
        "/" => do_index(),
        "/plan" => do_plan(),
        "/finger" => do_finger(),
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

// Generate the index.
fn do_index() -> Result<Response> {
    let finger = read_finger()?;
    let plan = read_plan()?;
    let data = types::FingerPlan { finger, plan };

    let mut hbars = Handlebars::new();
    hbars.register_template_file("index", "/files/templates/index.hbs")?;
    let out = hbars.render("index", &data)?;

    html_ok(out)
}

fn do_uc() -> Result<Response> {
    let finger = read_finger()?;
    let uc = types::FingerPlan {
        finger,
        plan: read_plan_md()?,
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
fn do_finger() -> Result<Response> {
    let finger_json = read_finger()?;
    let mut data = HashMap::new();
    data.insert("finger", finger_json);

    let mut hbars = Handlebars::new();
    hbars.register_template_file("index", "/files/templates/finger.hbs")?;
    let msg = hbars.render("index", &data)?;
    html_ok(msg)
}
/// Generate the plan page
fn do_plan() -> Result<Response> {
    let plan_text = read_plan()?;
    let mut data = HashMap::new();
    data.insert("plan", plan_text);

    let mut hbars = Handlebars::new();
    hbars.register_template_file("index", "/files/templates/plan.hbs")?;
    let msg = hbars.render("index", &data)?;
    html_ok(msg)
}

fn read_finger() -> Result<types::Finger> {
    let finger_text = std::fs::read_to_string(FINGER_PATH)?;
    let finger_data: types::Finger = serde_json::from_str(&finger_text)?;

    Ok(finger_data)
}

/// Read the plan file and convert to HTML
fn read_plan() -> Result<String> {
    let plan_text = std::fs::read_to_string(PLAN_PATH)?;
    Ok(md_to_html(&plan_text))
}

fn read_plan_md() -> Result<String> {
    let plan_text = std::fs::read_to_string(PLAN_PATH)?;
    Ok(plan_text)
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
