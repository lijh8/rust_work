fn main() {
    let haystack = vec![1, 2, 3, 2, 3, 4, 5];
    let needle = 2;

    for (index, &value) in haystack.iter().enumerate() {
        if value == needle {
            println!("Found {} at index: {}", needle, index);
        }
    }
}
