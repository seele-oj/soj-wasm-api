use js_sys::Promise;
use syntect::highlighting::ThemeSet;
use syntect::html::css_for_theme_with_class_style;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use once_cell::sync::Lazy;
use js_sys::Function;
use web_sys::{window, Element};


use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

static PS: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn format_code(code: &str, lang: &str) -> String {
    let syntax = PS.find_syntax_by_extension(lang).unwrap();
    let mut html_generator =
        ClassedHTMLGenerator::new_with_class_style(syntax, &PS, ClassStyle::Spaced);
    for line in LinesWithEndings::from(code) {
        html_generator.parse_html_for_line_which_includes_newline(line);
    }
    html_generator.finalize()
}

#[wasm_bindgen]
pub fn render_markdown(body: &str) -> String {
    markdown::to_html_with_options(
        body,
        &markdown::Options::gfm()
    ).unwrap()
}

#[wasm_bindgen]
pub fn problem_description(orig: String, id: i32, callback: Function) {
    spawn_local(async move {
        let url = format!("{}/problems/{}.md", orig, id);
        let result = reqwest::get(&url).await;
        match result {
            Ok(resp) => {
                match resp.text().await {
                    Ok(text) => {
                        let js_text = JsValue::from_str(&text);
                        let _ = callback.call1(&JsValue::NULL, &js_text);
                    }
                    Err(e) => {
                        let err_msg = JsValue::from_str(&format!("text read error: {:?}", e));
                        let _ = callback.call1(&JsValue::NULL, &err_msg);
                    }
                }
            }
            Err(e) => {
                let err_msg = JsValue::from_str(&format!("request error: {:?}", e));
                let _ = callback.call1(&JsValue::NULL, &err_msg);
            }
        }
    });
}