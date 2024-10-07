// macro_rules! type_name2 {
//     ($arg:expr) => {{
//         fn type_name2<T: ?Sized>(_: &T) -> &str {
//             std::any::type_name::<T>()
//         }
//         type_name2(&$arg)
//     }};
// }

//

fn foo<'a, 'b>(a: &'a String, b: &'b String) -> &'a String {
    // if a.len() > b.len() {
    //     a
    // } else {
    //     b
    // }
    a
}

fn main() {
    let a = String::from("abc");
    let b = String::from("abcdef");
    let c;
    {
        c = foo(&b, &a);
    }
    dbg!(&c);
}
