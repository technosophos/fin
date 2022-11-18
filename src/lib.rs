use anyhow::Result;
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
            .status(400)
            .body(Some("Not Found".into()))?),
    }
}

// Generate the index.
fn do_index() -> Result<Response> {
    let finger = read_finger()?;
    let plan = read_plan()?;

    let msg = format!("{}\n\nPlan:\n{}\n", finger, plan);

    Ok(http::Response::builder()
        .status(200)
        .body(Some(msg.into()))?)
}
/// Generate the finger page
fn do_finger() -> Result<Response> {
    let finger_text = read_finger()?;
    Ok(http::Response::builder()
        .status(200)
        .body(Some(finger_text.into()))?)
}
/// Generate the plan page
fn do_plan() -> Result<Response> {
    let plan_text = read_plan()?;
    Ok(http::Response::builder()
        .status(200)
        .body(Some(plan_text.into()))?)
}

fn read_finger() -> Result<String> {
    let finger_text = std::fs::read_to_string(FINGER_PATH)?;
    Ok(finger_text)
}

fn read_plan() -> Result<String> {
    let plan_text = std::fs::read_to_string(PLAN_PATH)?;
    Ok(plan_text)
}
