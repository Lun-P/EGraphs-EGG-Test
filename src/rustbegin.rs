pub fn printing() {
    println!("Hello World!");

    let mut x: i8 = -23;
    println!("{x} {}: {x:#b}, {x:#o}, {x:#x}", "in multiple formats");

    x *= -1;
    println!(
        "and positive: 0d{x} = {x:>#00$b}, {x:#o}, {x:#x}",
        size_of_val(&x) * 8 + 2 // Alternatively: 0b{x:0>binarysize$b}, where binarysize can be
                                // set here as kwarg or above as local variable
    );

    println!("pi is approximately {:.3}", 3.141592);

    eprintln!("Nothing went wrong, just checking io::stderr");

    #[derive(Debug)]
    struct Printstruct<'a>(i32, &'a str);
    let ps: Printstruct = Printstruct(3, "Hello world again!");

    println!("Debugged struct: {:#?}", &ps);

    use std::fmt;
    impl fmt::Display for Printstruct<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for _ in 0..self.0 {
                write!(f, "{} ", self.1)?
            }
            Ok(())
        }
    }
    println!("Printed struct: {}", &ps);
}
