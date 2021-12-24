use std::fs;

// TODO: Have this return a standard error. Same result as call_application.
pub fn serve_file(path: &str) -> Option<Vec<u8>> {
    let contents = fs::read(path).expect("could not read file");

    Some(contents)
}
