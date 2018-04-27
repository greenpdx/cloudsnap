use pulldown_cmark::{html, Options, Parser, OPTION_ENABLE_TABLES};

pub fn markdown_to_html(content: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    let mut capacity = String::with_capacity(content.len() * 3 / 2);
    let result = Parser::new_ext(content, opts);
    html::push_html(&mut capacity, result);
    capacity
}