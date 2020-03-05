#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn dist_from_origin(&self) -> f64 {
        let sum_of_squares = self.x.pow(2) + self.y.pow(2);
        (sum_of_squares as f64).sqrt()
    }
}

impl Point {
    fn translate_pt(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
}

// fn max(a: i32, b: i32) -> i32 {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }



fn main() {
    
    let p1 = Point {
        x: 3,
        y: 5
    };
    
    let mut p2 = p1.clone();

//Testing

    println!("point inc x: {}, y: {}", p2.x, p2.y); 
    println!("point's distance from origin: {}", p2.dist_from_origin());
    p2.translate_pt(5, 9);
    println!("Transforming point to: {:#?}",p2);
}
