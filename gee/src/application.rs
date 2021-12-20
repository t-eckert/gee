use crate::environ::Environ;

pub fn call(environ: Environ) {
    println!("Calling application.");
    println!("{}", environ);
}
