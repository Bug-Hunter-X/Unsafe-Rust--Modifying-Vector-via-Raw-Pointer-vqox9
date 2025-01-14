fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // safer way to modify elements
    println!("{:?}", v);
}