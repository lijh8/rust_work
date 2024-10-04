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
    let haystack = [1, 2, 3, 3, 3, 4, 5];
    let needle = 3;

    let begin = haystack.partition_point(|x| x < &needle);
    let end = haystack.partition_point(|x| x <= &needle);
    if begin == end {
        dbg!(format_args!("not found, {}, {}", begin, end));
    } else {
        dbg!(format_args!("found, {}, {}", begin, end));
    }
}
