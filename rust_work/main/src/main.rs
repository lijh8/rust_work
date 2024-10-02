macro_rules! println2 {
    ($($args:tt)*) => {
        let message = format!("{}:{}: {}", file!(), line!(), format_args!($($args)*));
        println!("{message}");
    };
}

fn main() {
    // Result

    let result: Result<i32, &str> = Ok(123);
    // let result: Result<i32, &str> = Err("Error message");

    if result.is_ok() {
        println2!("result: {}", result.unwrap());
    }
    if result.is_err() {
        println2!("is_err: {}", result.unwrap_err());
    }

    if let Ok(value) = result {
        println2!("result: {}", value);
    }
    if let Err(e) = result {
        println2!("is_err: {e}");
    }

    // Option

    let option: Option<i32> = Some(123);
    // let option: Option<i32> = None;

    if option.is_some() {
        println2!("option: {}", option.unwrap());
    }
    if option.is_none() {
        println2!("is_none");
    }

    if let Some(value) = option {
        println2!("option: {}", value);
    }
    if let None = option {
        println2!("is_none");
    }
}
