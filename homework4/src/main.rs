enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Timeable {
    fn duration(&self) -> u32;
}

impl Timeable for TrafficLight {
    fn duration(&self) -> u32 {
        match *self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;
    println!("Red light duration: {} seconds", red.duration());
    println!("Yellow light duration: {} seconds", yellow.duration());
    println!("Green light duration: {} seconds", green.duration());
}
