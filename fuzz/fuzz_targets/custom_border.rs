#![no_main]
use libfuzzer_sys::fuzz_target;

use boxx::{BorderComponents, BorderStyle, Boxx};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Boxx::builder()
            .border_style(BorderStyle::Custom(BorderComponents {
                top_left: "x".to_string(),
                top_right: "x".to_string(),
                bottom_left: "x".to_string(),
                bottom_right: "x".to_string(),
                horizontal: s.to_string(),
                vertical: s.to_string(),
            }))
            .build()
            .display(s);
    }
});
