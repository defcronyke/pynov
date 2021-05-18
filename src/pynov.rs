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

use crate::libnov::ViewKind;
use pyo3::prelude::*;

#[cfg(feature = "python")]
use pyo3::wrap_pyfunction;

#[cfg(feature = "python")]
fn init_python_mod(m: &PyModule) -> PyResult<()> {
    use crate::python::*;

    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(run_file, m)?)?;

    Ok(())
}

fn init_view_mod(m: &PyModule) -> PyResult<()> {
    use crate::view::*;

    m.add_function(wrap_pyfunction!(new, m)?)?;

    Ok(())
}

#[pymodule]
fn pynov(_py: Python, m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "python")]
    {
        let python_mod = PyModule::new(_py, "python")?;
        init_python_mod(python_mod)?;
        m.add_submodule(python_mod)?;
    }

    let view_mod = PyModule::new(_py, "view")?;
    init_view_mod(view_mod)?;
    m.add_submodule(view_mod)?;

    #[pyfn(m, "main")]
    fn main_py(_py: Python) -> PyResult<String> {
        let out = main();
        Ok(out)
    }

    #[pyfn(m, "libmain")]
    fn libmain_py(_py: Python) -> PyResult<String> {
        let out = libmain();
        Ok(out)
    }

    #[pyfn(m, "nov")]
    fn nov_py(_py: Python) -> PyResult<String> {
        let out = nov();
        Ok(out)
    }

    Ok(())
}

fn main() -> String {
    let res = format!("pynov.main() called");

    println!("{}", res);

    let _ = libnov::main(Ok(()), |view: &mut libnov::view::View, res| {
        println!("libnov::main() called from python");

        let view_name = view.get_name();

        println!("[ {} available ]\n", view_name);

        #[cfg(feature = "python")]
        if view.feature_python.is_some() {
            println!(
                "[ {} feature available ]: {}",
                view_name,
                view.feature_python.as_ref().unwrap()
            );
        }

        // This must run last.
        libnov::window::Window::new(libnov::conf::load(None)?).open_image(res.clone());

        res
    });

    res
}

fn libmain() -> String {
    let res = format!("pynov.libmain() called");

    println!("{}", res);

    let _ = libnov::main(Ok(()), |_view: &mut libnov::view::View, res| {
        println!("libnov::main() called from python");

        res
    });

    res
}

fn nov() -> String {
    let res = format!("pynov.nov() called");

    println!("{}", res);

    res
}
