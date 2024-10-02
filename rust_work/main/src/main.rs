fn f<'a>() -> &'a str {
    "abc".to_string().as_str()
}

fn main() {
    let a = f();
}
