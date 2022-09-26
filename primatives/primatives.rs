// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let Point { x:x1, y:y1} = self.top_left;
        let Point { x:x2, y:y2} = self.bottom_right;

        ((x1-x2)*(y1-y2)).abs()

    }
}

fn main() {
   let top_left = Point { x: 0.0, y:4.0};
   let bottom_right = Point { x: 4.0, y:0.0};

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: top_left,
        bottom_right: bottom_right,
    };

    println!("{}", _rectangle.area());

    let _square = square(Point {x:0.0,y:4.0}, 2.0);

    println!("{}", _square.area());

}

fn square(top_left:Point, edge_size:f32) -> Rectangle{
    let bottom_right = Point { x: top_left.x +edge_size, y:top_left.y-edge_size };
    return Rectangle { top_left: top_left, bottom_right : bottom_right};
}
