fn main() {
    // 燈號 Start //
    let red: TrafficLight = Light::new(LightColor::RED);
    println!("red duration: {}", red.duration());
    let yellow: TrafficLight = Light::new(LightColor::YELLOW);
    println!("yellow duration: {}", yellow.duration());
    let green: TrafficLight = Light::new(LightColor::GREEN);
    println!("green duration: {}", green.duration());
    // 燈號 End //

    // 加總 Start //
    let result = sum(&[0, u32::MAX]);

    match result {
        Some(p) => println!("total: {}", p),
        None => println!("total: None."),
    };

    let result = sum(&[1, u32::MAX]);

    match result {
        Some(p) => println!("total: {}", p),
        None => println!("total: None."),
    };
    // 加總 End //

    // 形狀面積 End //
    compute(Square{});
    compute(Triangle{});
    compute(Circle{});
    // 形狀面積 End //
}

// 燈號 Start //
struct TrafficLight { color: LightColor }

trait Light {
    fn new(light_color: LightColor) -> Self;

    fn duration(&self) -> u8;
}

impl TrafficLight {}

impl Light for TrafficLight {
    fn new(color: LightColor) -> TrafficLight {
        TrafficLight { color: color }
    }

    fn duration(&self) -> u8 {
        let _duration: u8 = match &self.color {
            LightColor::RED => 10,
            LightColor::YELLOW => 2,
            LightColor::GREEN => 15,
        };

        return _duration;
    }
}

enum LightColor {
    RED,
    YELLOW,
    GREEN
}
// 燈號 End //

// 加總 Start //
fn sum(numbers: &[u32]) -> Option<u32> {
    let mut result: u64 = 0;

    for number in numbers {
        result = result + (*number as u64);
    }

    if result > u32::MAX as u64 {
        return None;
    }

    return Some(result as u32);
}
// 加總 End //

trait Shape {
    fn area(&self) -> String;
}

struct Square {}
impl Square {}
struct Triangle {}
impl Triangle {}
struct Circle {}
impl Circle {}

impl Shape for Square {
    fn area(&self) -> String {
        return String::from("10");
    }
}

impl Shape for Triangle {
    fn area(&self) -> String {
        return String::from("20");
    }
}

impl Shape for Circle {
    fn area(&self) -> String {
        return String::from("30");
    }
}

fn compute<T: Shape>(shape: T) {
    println!("Area: {}", shape.area());
}