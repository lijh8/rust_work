pub mod add2;

#[cfg(test)]
pub mod tests3; // comment this to run integration test

pub fn add(a: i32, b: i32) -> i32 {
    dbg!(format_args!("{}", a + b));
    a + b
}
