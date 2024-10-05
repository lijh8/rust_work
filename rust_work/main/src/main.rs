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
    let mut c = 10;
    c = c + 1;
    c = c + 2;
    dbg!(format_args!("{}", c));
}
