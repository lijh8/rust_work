// macro_rules! type_name2 {
//     ($arg:expr) => {{
//         fn type_name2<T: ?Sized>(_: &T) -> &str {
//             std::any::type_name::<T>()
//         }
//         type_name2(&$arg)
//     }};
// }

//

fn main() {
    {
        let a = String::from("abc");
        let b = String::from("abc");
        let c = a == b;
        a;
        b;
        // let a2 = a;
        // let b2 = b;
        dbg!(format_args!("{}, {}, {}", a, b, c));
    }
    {
        let a = "abc";
        let b = "abc";
        let c = a == b;
        // let a2 = a;
        // let b2 = b;
        dbg!(format_args!("{}, {}, {}", a, b, c));
    }
}
