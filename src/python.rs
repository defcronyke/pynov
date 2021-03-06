/*  Copyright (c) 2021 Jeremy Carter <jeremy@jeremycarter.ca>

    All uses of this project in part or in whole are governed
    by the terms of the license contained in the file titled
    "LICENSE" that's distributed along with the project, which
    can be found in the top-level directory of this project.

    If you don't agree to follow those terms or you won't
    follow them, you are not allowed to use this project or
    anything that's made with parts of it at all. The project
    is also	depending on some third-party technologies, and
    some of those are governed by their own separate licenses,
    so furthermore, whenever legally possible, all license
    terms from all of the different technologies apply, with
    this project's license terms taking first priority.
*/

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
use libnov::python;

#[cfg(feature = "python")]
#[pyfunction]
pub fn init() -> String {
    let res = format!("pynov.python.init() called");

    println!("{}", res);

    let _ = python::init();

    res
}

#[cfg(feature = "python")]
#[pyfunction]
pub fn run_file(
    filepath: Option<&str>,
    globals: Option<Vec<&str>>,
    locals: Option<Vec<&str>>,
) -> String {
    let res = format!("pynov.python.run_file() called");

    println!("{}", res);

    let f = filepath.unwrap_or_default();
    let g = globals.unwrap_or_default();
    let l = locals.unwrap_or_default();

    let _ = python::run_file(f, g, l);

    res
}
