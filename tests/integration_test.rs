// Integration test for SigmaOS-kernel-memory-accounting
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use kernel_memory_accounting as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
