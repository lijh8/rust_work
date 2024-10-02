// fn type_name2<T>(_: &T) -> &str {
//     std::any::type_name::<T>()
// }

macro_rules! type_name2 {
    ($val:expr) => {{
        fn type_name2<T>(_: &T) -> &str {
            std::any::type_name::<T>()
        }
        type_name2(&$val)
    }};
}

fn main() {
    let name = "abc".to_string();
    println!("{}", type_name2!(10 + 1));
    println!("{}", type_name2!("abc".to_string()));
    println!("{}", type_name2!(name)); // do not take ownership
    let name = name; // still in use
}
