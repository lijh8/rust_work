macro_rules! dbg2 {
    ($($arg:tt)*) => {
        dbg!(format!($($arg)*))
    };
}

macro_rules! println2 {
    ($($args:tt)*) => {
        println!("{}:{}: {}", file!(), line!(), format_args!($($args)*));
    };
}

fn main() {
    let name = "abc".to_string();
    let num = 123;

    dbg2!("{}, {}", name, num);
    dbg!(format!("{}, {}", name, num));
    println2!("{}, {}", name, num);

    let name = name; // ownership remains and was not taken
}
