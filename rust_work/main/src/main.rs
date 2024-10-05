fn main() {
    // let option: Option<i32> = Some(123);
    let option: Option<i32> = None;

    if let Some(a) = option {
        println!("{}", a);
    }
    println!("test");

    // if let Some(value) = option {
    //     println!("option: {}", value);
    // }
    // if let None = option {
    //     println!("is_none");
    // }

    // if option.is_some() {
    //     println!("option: {}", option.unwrap());
    // }
    // if option.is_none() {
    //     println!("is_none");
    // }
}
