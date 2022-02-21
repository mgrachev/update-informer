use std::time::Duration;
use update_informer::{registry::Crates, Check, UpdateInformer};

fn main() {
    let pkg_name = "update-informer";
    let current_version = "0.1.0";

    let informer = UpdateInformer::new(Crates, pkg_name, current_version).interval(Duration::ZERO);

    if let Ok(Some(new_version)) = informer.check_version() {
        println!("A new release of {pkg_name} is available: v{current_version} -> {new_version}");
    }
}
