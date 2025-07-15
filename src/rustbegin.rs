use std::fmt;

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

pub fn types() {
    let testarr: [i32; 100] = [25; 100];
    println!("Test array: {:?}", testarr);

    let arr_to_slice: &[i32] = &testarr;
    println!("Array as slice: {:?}", arr_to_slice);

    let arr_to_slice_smaller: &[i32] = &testarr[20..23];
    println!("Now with fewer elements: {:?}", arr_to_slice_smaller);

    #[derive(Debug)]
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let p1 = Point3D { x: 4, y: 5, z: 0 };
    let p2 = Point3D { x: 40, ..p1 }; // Use traits from p1
    println!("{:?} and {:?}", p1, p2);

    enum Points {
        Point3D(Point3D),
        Point2D { x: i32, y: i32 },
        Point1D(i32),
    }
    type P = Points;

    let a = P::Point3D(Point3D { x: 1, y: 2, z: 3 });
    match a {
        P::Point3D(Point3D { x, y, z }) => println!("This is a Point3D: {x}, {y}, {z}"),
        P::Point2D { x, y } => println!("This is a Point2D: {x}, {y}"),
        P::Point1D(x) => println!("This is a number: {x}"),
    }
}
