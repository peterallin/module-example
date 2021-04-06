use crate::foo;
use crate::baz;

pub fn bar() {
    foo::foo();
    baz::baz();
    println!("bar")
}