#![no_main]

use risc0_zkvm;

extern "C" {
    pub fn load_data() -> u8;
}

#[no_mangle]
unsafe fn verify() {
    let _ = load_data();
}
