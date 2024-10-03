macro_rules! type_name2 {
    ($arg:expr) => {{
        fn type_name2<T: ?Sized>(_: &T) -> &str {
            std::any::type_name::<T>()
        }
        type_name2(&$arg)
    }};
}

//

fn main() {
    let v = vec![1, 2, 3];
    let a = [1, 2, 3];

    dbg!(format!("{}", type_name2!(v)));
    dbg!(format!("{}", type_name2!(a)));
    dbg!(format!("{}", type_name2!(&v)));
    dbg!(format!("{}", type_name2!(&a)));
    dbg!(format!("{}", type_name2!(&v[..])));
    dbg!(format!("{}", type_name2!(&a[..])));

    dbg!(format!("{}", type_name2!(v[..])));
    dbg!(format!("{}", type_name2!(a[..])));

    let a2 = a;
    let v2 = v;
}
