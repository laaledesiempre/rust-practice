// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

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

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    let testing_rectangle= Rectangle {
        top_left: Point { x: 5.0, y:2.0},
        bottom_right: Point {x: 7.0, y: 0.0}
        
    };
    let rectanglearea= rect_area(&testing_rectangle);
    println!("{}",rectanglearea);
    let new_square= square(&Point {x: 2.0, y:5.0}, 2.0);
    println!("new square top left: {},{}. new square bottom right: {},{}", new_square.top_left.x, new_square.top_left.y, new_square.bottom_right.x, new_square.bottom_right.y);
}

fn rect_area(rect: &Rectangle)->f32{
    let Rectangle {top_left:left, bottom_right:right}=rect;
    let side =left.y-right.y;
    let base =right.x-left.x;
    side*base
}

fn square(point: &Point, size: f32)-> Rectangle{
    Rectangle{
        top_left: Point {x:point.x,y:point.y},
        bottom_right: Point {x:point.x+size,y:point.y-size },
    }
}
