use nalgebra::Vector2;
use proconio::input;

fn main() {
    input! {
        a: (f64, f64),
        b: (f64, f64),
        c: (f64, f64),
    }

    let ba = Vector2::new(a.0 - b.0, a.1 - b.1);
    let bc = Vector2::new(c.0 - b.0, c.1 - b.1);
    let ca = Vector2::new(a.0 - c.0, a.1 - c.1);
    let cb = -1. * bc;
    if ba.dot(&bc) < 0. {
        println!("{}", ba.norm());
    } else if ca.dot(&cb) < 0. {
        println!("{}", ca.norm());
    } else {
        println!("{}", (ba[0] * bc[1] - ba[1] * bc[0]).abs() / bc.norm());
    }
}
