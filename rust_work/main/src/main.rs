macro_rules! println23 {
    ($($args:tt)*) => {{
        let message = format!("{}:{}: {}", file!(), line!(), format_args!($($args)*));
        println!("{message}");
    }};
}

macro_rules! println2 {
    ($($args:tt)*) => {
        println!("{}:{}: {}", file!(), line!(), format_args!($($args)*));
    };
}

use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println2!("First element of the slice: {}", slice[0]);
    println2!("The slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0.
    println2!("First element of the array: {}", xs[0]);
    println2!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
    println2!("Number of elements in array: {}", xs.len());

    // Arrays are stack allocated.
    println2!("Array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices.
    println2!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println2!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println2!("{}: {}", i, xval),
            None => println2!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array causes compile time error.
    //println2!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error.
    //println2!("{}", xs[..][5]);
}
