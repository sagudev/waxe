#[macro_use]
extern crate mozjs;
/// Engine extender
pub mod jex;
/// Engine runner
pub mod rtx;
mod sm;
/// Trio
pub mod trio;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
