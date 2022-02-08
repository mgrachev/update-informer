use std::time::Duration;
use update_informer::{registry::PyPI, Check, UpdateInformer};

fn main() {
    let pkg_name = "filprofiler";
    let current_version = "2022.1.0";

    let informer = UpdateInformer::new(PyPI, pkg_name, current_version, Duration::default());

    if let Ok(Some(new_version)) = informer.check_version() {
        println!("A new release of {pkg_name} is available: v{current_version} -> {new_version}");
    }
}
