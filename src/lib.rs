use std::collections::HashMap;

use anyhow::Result;
use handlebars::Handlebars;
use pulldown_cmark as markdown;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

const FINGER_PATH: &str = "/files/finger.md";
const PLAN_PATH: &str = "/files/plan.md";

/// A simple Spin HTTP component.
#[http_component]
fn finger(req: Request) -> Result<Response> {
    match req.uri().path() {
        "/" => do_index(),
        "/plan" => do_plan(),
        "/finger" => do_finger(),
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

    let mut data = HashMap::new();
    data.insert("finger", finger);
    data.insert("plan", plan);

    let mut hbars = Handlebars::new();
    hbars.register_template_file("index", "/files/templates/index.hbs")?;
    let out = hbars.render("index", &data)?;

    html_ok(out)
}
/// Generate the finger page
fn do_finger() -> Result<Response> {
    let finger_text = read_finger()?;
    let mut data = HashMap::new();
    data.insert("finger", finger_text);

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

/// Read the finger file and convert to HTML.
fn read_finger() -> Result<String> {
    let finger_text = std::fs::read_to_string(FINGER_PATH)?;
    Ok(md_to_html(&finger_text))
}

/// Read the plan file and convert to HTML
fn read_plan() -> Result<String> {
    let plan_text = std::fs::read_to_string(PLAN_PATH)?;
    Ok(md_to_html(&plan_text))
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
