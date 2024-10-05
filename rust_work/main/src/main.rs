macro_rules! type_name2 {
    ($arg:expr) => {{
        fn type_name2<T: ?Sized>(_: &T) -> &str {
            std::any::type_name::<T>()
        }
        type_name2(&$arg)
    }};
}

fn main() {
    {
        let arr = vec![1, 2, 3, 4];
        let vec: Vec<_> = arr.iter().map(|v| v.to_string()).collect();
        dbg!(format_args!("{:?}", vec));
    }

    {
        let arr = vec![1, 2, 3];
        let vec: Vec<String> = arr
            .iter()
            .flat_map(|&v| std::iter::once(v.to_string()))
            .collect();

        dbg!(format_args!("{:?}", vec));
    }

    // The purpose of flattening in flat_map is to handle cases where each element of
    // an iterator maps to a collection (like another iterator, vector, or string),
    // and you want to merge all those collections into a single iterator.
    // Essentially, flat_map combines mapping and flattening into one operation.
    //
    // It can make nested data <Vec<Vec<char>> into a flat data <Vec<char>> .

    {
        let nested_vec: Vec<Vec<char>> = vec![vec!['a', 'b'], vec!['c', 'd'], vec!['e']];
        dbg!(format_args!(
            "{:?}, {:?}",
            nested_vec,
            type_name2!(nested_vec)
        ));
        let flat_vec: Vec<char> = nested_vec
            .into_iter()
            .flat_map(|inner_vec| inner_vec)
            .collect();
        dbg!(format_args!("{:?}, {:?}", flat_vec, type_name2!(flat_vec)));
    }
}
