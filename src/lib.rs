use syntect::highlighting::ThemeSet;
use syntect::html::css_for_theme_with_class_style;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use once_cell::sync::Lazy;

use wasm_bindgen::prelude::*;

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