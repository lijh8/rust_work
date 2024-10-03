macro_rules! type_name2 {
    ($arg:expr) => {{
        fn type_name2<T>(_: &T) -> &str {
            std::any::type_name::<T>()
        }
        type_name2(&$arg)
    }};
}

fn main() {
    let number = 5;
    let text = "Hello";
    let text2 = "Hello".to_string();

    dbg!(format!("{}", type_name2!(&number)));
    dbg!(format!("{}", type_name2!(&text)));
    dbg!(format!("{}", type_name2!(&text2)));
}
