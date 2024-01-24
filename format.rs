use std::fmt::{Formatter, Display, Result}; // can pull out items by name

/* This demonstrates the usage of the format! macro directly and import by name. */

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let lat_c = if self.lat >= 0.0 { "N" } else { "S" };
        let lon_c = if self.lon >= 0.0 { "E" } else { "W" };
        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

fn main() {
    for city in [
        City { name: "Monmouthy", lat: 51.8167, lon: -3.0167 },
        City { name: "Little Yorky", lat: 40.7127, lon: -74.0059 },
    ] {
        println!("{}", city);
    }

    println!("{}", format!("{}", "Boo")); // using the format string directly
}