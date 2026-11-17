// Integration test for SigmaOS-archlinux-themes-kde
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_archlinux_themes_kde as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
