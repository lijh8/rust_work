fn type_name2<'a, T>(_: T) -> &'a str {
    std::any::type_name::<T>()
}

fn main() {
    let name = "abc";
    let num = 123;
    dbg!(format!("{}", type_name2(name)));
    dbg!(format!("{}", type_name2(num + 1)));
}

/*
[main/src/main.rs:8:5] format!("{}", type_name2(name)) = "&str"
[main/src/main.rs:9:5] format!("{}", type_name2(num + 1)) = "i32"
*/
