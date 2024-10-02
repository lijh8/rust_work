macro_rules! println2 {
    ($($args:tt)*) => {
        let message = format!("{}:{}: {}", file!(), line!(), format_args!($($args)*));
        println!("{message}");
    };
}

fn main() {
    let name = "abc";
    let num = 123;
    println2!("{name}, {num}");
    dbg!(format!("{name}, {num}")); // dbg!() works in release too.
}

/*
main/src/main.rs:17: abc, 123
[main/src/main.rs:18:5] format!("{name}, {num}") = "abc, 123"
$

*/
