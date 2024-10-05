// use through package_name.
use test4::add;
use test4::add2::add2;

fn main() {
    let a = add(1, 2);
    let b = add2(1, 2);
    let mut c = 10;
    c = c + 1;
    c = c + 2;
    dbg!(format_args!("{}", a));
    dbg!(format_args!("{}", b));
}
