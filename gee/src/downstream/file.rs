use std::fs;

// TODO: Have this return a standard error. Same result as call_application.
pub fn serve_file(path: &str) -> Option<Vec<u8>> {
    let read_result = fs::read(path);

    match read_result {
        Ok(contents) => Some(contents),
        _ => None,
    }
}
