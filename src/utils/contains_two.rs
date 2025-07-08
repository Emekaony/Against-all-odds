pub fn contains_two(arr: &[i32]) {
    for item in arr.iter() {
        if *item == 2 {
            println!("Found 2");
        }
    }

    println!("2 is not here");
}
