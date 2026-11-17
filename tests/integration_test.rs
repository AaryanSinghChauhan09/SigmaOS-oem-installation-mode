//! Integration tests for SigmaOS-oem-installation-mode.

use sigmaos_oem_installation_mode_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
