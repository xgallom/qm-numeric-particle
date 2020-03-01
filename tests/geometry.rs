#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate qm_numeric_particle;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let mut matrix = qm_numeric_particle::geometry::dynamic_linear_object::
}
