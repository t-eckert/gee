use crate::environ::Environ;

pub fn call_application(environ: Environ) -> Option<Vec<u8>> {
    println!("Calling application.");
    println!("{}", environ);

    Some("Response from Python".as_bytes().to_owned())
}
