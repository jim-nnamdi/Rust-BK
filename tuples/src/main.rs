fn main() {
    let t = ("boi", 10);
    println!("{:?}", t);

    let t : (i32, &str) = (12, "bot");
    println!("{:?}", t.1);
}
