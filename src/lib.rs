mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn prime(n: u32) -> u32 {
    let mut max = 0u32;
    for i in 3..n {
        let mut j = 2u32;
        while j < i {
            if i % j == 0 {
                break;
            }
            j += 1;
        }
        if j == i && i > max {
            max = i;
        }
    }
    return max
} 