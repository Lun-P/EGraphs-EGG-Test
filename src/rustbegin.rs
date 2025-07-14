pub fn printing() {
    println!("Hello World!");
    let mut x: i8 = -23;
    println!("{x} {}: 0b{x:b}, 0o{x:o}, 0x{x:x}", " in multiple formats");
    x *= -1;
    println!("and positive: 0b{:>08b}, 0o{:o}, 0x{:x}", x, x, x);

    eprintln!("Nothing went wrong, just checking io::stderr");
}