use std::fs;

use crate::{environ::Environ, hashmap};
use pyo3::{prelude::*, types::PyTuple};

// TODO: break this function down into sub-functions. Doing so was giving me some lifetime errors...
pub fn call_application(environ: Environ) -> Option<Vec<u8>> {
    println!("Calling application.");
    println!("{}", environ);

    let code = fs::read_to_string("./app/app.py").expect("Cannot find Python file!");
    let filename = "app.py";
    let modulename = "app";
    let callablename = "print_environ";

    let fake_environ = hashmap!["a" => "b"];

    Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, &code, filename, modulename).expect("Cannot load module!");
        let callable = module.getattr(callablename).expect("Cannot load callable!");

        let args = PyTuple::new(py, &[fake_environ]);
        let response = callable.call1(args).expect("Cannot call callable!");
    });

    Some("Response from Python".as_bytes().to_owned())
}
