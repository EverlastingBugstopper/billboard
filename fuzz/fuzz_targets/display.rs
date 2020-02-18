#![no_main]
use libfuzzer_sys::fuzz_target;

use boxx::Boxx;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Boxx::default().display(s);
    }
});
