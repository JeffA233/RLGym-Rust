use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn to_string(&self) -> String;
}

struct Square {
    m_width: i64
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return (self.m_width * self.m_width) as f64
    }

    fn perimeter(&self) -> f64 {
        return (4*self.m_width) as f64
    }

    fn to_string(&self) -> String {
        return "square width: ".to_owned() + &self.m_width.to_string()
    }
}

struct Circle {
    m_radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        return (PI * self.m_radius * self.m_radius) as f64
    }

    fn perimeter(&self) -> f64 {
        return (2.* PI * self.m_radius) as f64
    }

    fn to_string(&self) -> String {
        return "circle radius ".to_owned() + &self.m_radius.to_string()
    }
}

fn main() {
    // size not known so it errors with or without dyn, sadge
    let my_shapes = Vec::<(Square, Circle)>::new();

    let square = Square{ m_width: 1};
    let circle = Circle{ m_radius: 1.};

    let square_str = square.to_string();
    let square_area = square.area();
    let square_perim = square.perimeter();
    println!("{square_str} - area: {square_area}; perimeter: {square_perim}");

    let circle_str = circle.to_string();
    let circle_area = circle.area();
    let circle_perim = circle.perimeter();
    println!("{circle_str} - area: {circle_area}; perimeter: {circle_perim}")
}
