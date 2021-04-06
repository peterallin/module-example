mod foo;
mod bar;

mod baz {
    pub fn baz() {
        println!("baz")
    }
}

fn main() {
    bar::bar();
}
