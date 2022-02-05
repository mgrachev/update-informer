use std::{env::args, time::Duration};
use update_informer::{registry::PyPI, Check, UpdateInformer};

fn main() {
    let pkg_name = args().nth(1).expect("Must provide package name.");
    let current_version = args().nth(2).expect("Must provide version.");
    let interval = Duration::from_secs(1);

    let informer = UpdateInformer::new(PyPI, pkg_name.clone(), current_version.clone(), interval);
    match informer.check_version() {
        Ok(Some(version)) => println!(
            "A new release of {pkg_name} is available: v{current_version} -> {new_version}",
            pkg_name = pkg_name,
            current_version = current_version,
            new_version = version,
        ),
        Ok(None) => println!("Didn't check version since timeout wasn't hit"),
        Err(e) => println!("An error occurred: {}", e),
    }
}
