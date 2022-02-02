use std::time::Duration;
use update_informer::{registry::PyPI, Check, UpdateInformer};

fn main() {
    let pkg_name = env!("PYPI_PKG_NAME");
    let current_version = env!("PYPI_PKG_VERSION");
    let interval = Duration::from_secs(1);

    let informer = UpdateInformer::new(PyPI, pkg_name, current_version, interval);
    if let Ok(Some(version)) = informer.check_version() {
        println!(
            "A new release of {pkg_name} is available: v{current_version} -> {new_version}",
            pkg_name = pkg_name,
            current_version = current_version,
            new_version = version,
        );
    }
}
