pub mod md_helper;
pub fn read_to_bytes(path: &str) -> Vec<u8> {
    if let Ok(content) = std::fs::read(path) {
        content
    } else {
        Vec::new()
    }
}

pub fn read_to_string(path: &str) -> String {
    if let Ok(content) = std::fs::read_to_string(path) {
        content
    } else {
        read_404()
    }
}

fn read_404() -> String {
    if let Ok(content) = std::fs::read_to_string("assert/html/404.html") {
        content
    } else {
        "Something went wrong reading the file".to_string()
    }
}
