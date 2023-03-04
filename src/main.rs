fn main() {
    let v1 = u64::MAX;
    println!("{}", v1);

    let mut v3 = usize::MAX;

    println!("{}", v3);

    v3 = v1 as usize + 5;

    println!("{}", v3);
}
