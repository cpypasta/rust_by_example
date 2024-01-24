use std::fmt;

struct Structure(i32); // a tuple of one element

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?; // if error, returns error, else continue

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]") // no semi-colon on last line; returns result; ends multi-line ? pattern
    }
}

fn main() {
    println!("Basic tuple structure: {}", Structure(54));

    let point = Point2D { x: 2.3, y: 5.4 };
    println!("Point2D");
    println!("Debug: {:?}", point);
    println!("Display: {}", point);

    let v = List(vec![2,3,5,4]);
    println!("{}", v);
}