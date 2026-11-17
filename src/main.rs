//! SigmaOS-oem-installation-mode — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_oem_installation_mode_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-oem-installation-mode: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-oem-installation-mode: ok");
}
