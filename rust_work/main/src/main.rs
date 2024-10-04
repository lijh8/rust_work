// macro_rules! type_name2 {
//     ($arg:expr) => {{
//         fn type_name2<T: ?Sized>(_: &T) -> &str {
//             std::any::type_name::<T>()
//         }
//         type_name2(&$arg)
//     }};
// }

//

// let name = "abc"
// let num = 123

// dbg!(format!("{}, {}", name, num));
// dbg2!("{}, {}", name, num);
macro_rules! dbg2 {
    ($($args:tt)*) => {
        dbg!(format_args!($($args)*))
    };
}

// println2!("{}, {}", name, num);
macro_rules! println2 {
    ($($args:tt)*) => {
        println!("{}:{}: {}", file!(), line!(), format_args!($($args)*));
    };
}

//

fn main() {
    let text = "123".to_string();
    let mut num: i32 = 0;
    // if let Ok(x) = text.parse::<i32>() {
    //     num = x;
    //     // dbg!(format!("{}, {}", text, num));
    // };
    if text.parse::<i32>().is_ok() {
        num = text.parse::<i32>().unwrap();
    }

    dbg!(format!("{}, {}", text, num));
    dbg!(format!("{}, {}", text, num));
    dbg2!("{:?}, {:?}", text, num);
    dbg2!("{:?}, {:?}", text, num);
    println2!("{:?}, {:?}", text, num);
    println2!("{:?}, {:?}", text, num);
}
