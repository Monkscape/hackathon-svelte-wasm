use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

extern crate web_sys;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! log {
    ( $( $t:tt)* ) => {
        web_sys::console::log_1(&format!( $( $t)* ).into());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DogResponse {
    pub message: Vec<String>,
}

#[wasm_bindgen]
pub async fn run(breed: String, number: u8) -> Result<JsValue, JsValue> {
    utils::set_panic_hook();

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://dog.ceo/api/breed/{}/images/random/{}", breed, number);
    log!("Hitting endpoint {}", url);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    log!("Response Status: {}", resp.status());
    match JsFuture::from(resp.json()?).await {
        Ok(json) => {
            let response: DogResponse = match json.into_serde() {
                Ok(response) => response,
                Err(_) => DogResponse {message: vec![]}
            };
            log!("Response: {:?}", response);
            Ok(JsValue::from_serde(&response).unwrap())
        }
        Err(_) => {
            panic!("Unable to handle future");
        }
    }
}

#[wasm_bindgen(start)]
pub fn performance_timing() {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");

    log!("the current time (in ms) is {}", performance.now());

    let start = perf_to_system(performance.timing().request_start());
    let end = perf_to_system(performance.timing().response_end());

    log!("request started at {}", humantime::format_rfc3339(start));
    log!("request ended at {}", humantime::format_rfc3339(end));
}

fn perf_to_system(amt: f64) -> SystemTime {
    let secs = (amt as u64) / 1_000;
    let nanos = ((amt as u32) % 1_000) * 1_000_000;
    UNIX_EPOCH + Duration::new(secs, nanos)
}

