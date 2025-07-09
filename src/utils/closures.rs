pub fn apply(f: fn(&mut i32) -> i32, a: &mut i32) -> i32 {
    f(a)
}
