macro_rules! type_name2 {
    ($arg:expr) => {{
        fn type_name2<T>(_: &T) -> &str {
            std::any::type_name::<T>()
        }
        type_name2(&$arg)
    }};
}

// rust-by-example/flow_control/let_else.html, 8.7:

fn main() {
    let text = "123";
    let parse = text.parse::<i32>();
    let Ok(num) = parse.as_ref() else {
        dbg!(format!("{}", parse.as_ref().unwrap_err()));
        return;
    };
    dbg!(format!("{}, {}", text, num));

    let text = "abc";
    let parse = text.parse::<i32>();
    let Ok(num) = parse.as_ref() else {
        dbg!(format!("{}", parse.as_ref().unwrap_err()));
        return;
    };

    dbg!(format!("{}, {}", text, num));
}

/*
[main/src/main.rs:17:5] format!("{}, {}", text, num) = "123, 123"
[main/src/main.rs:22:9] format!("{}", parse.as_ref().unwrap_err()) = "invalid digit found in string"
 */
