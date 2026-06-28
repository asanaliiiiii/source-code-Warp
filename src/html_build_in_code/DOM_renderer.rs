use std::collections::HashMap;

pub fn render_html_tree(html: &str, variables: &HashMap<String, String>) {
    let mut parsed_html = html.to_string();
    for (key, value) in variables {
        parsed_html = parsed_html.replace(&format!("{{{}}}", key), value);
    }
    println!("\n🌐 [Warp Web Context Layout Virtual-DOM Stack Engine]:");
    println!("{}", parsed_html.trim());
}