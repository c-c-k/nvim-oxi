//! Tests about the `#[nvimo::test]` macro.

#[should_panic]
#[nvimo::test]
fn panic_is_propagated() {
    panic!();
}

#[nvimo::test]
fn printing_to_stderr_is_ok() {
    eprintln!("AA!");
}
