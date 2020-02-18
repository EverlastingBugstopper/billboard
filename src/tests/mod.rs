#[cfg(test)]
use crate::{BorderStyle, Boxx};

#[test]
fn can_display() {
    let x = Boxx::default().as_str("Hello, World");
    println!("{}", x);
    assert!(true);
}
