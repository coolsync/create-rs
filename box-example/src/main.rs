use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// A Rectangle can be specified by where its top left and bottom right
// corners are in space
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // Allocate this point on the heap, and return a pointer to it
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    let point = origin();
    let rect = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    println!("point {} bytes on the stack", mem::size_of_val(&point));
    println!("rect {} bytes on the stack", mem::size_of_val(&rect));

    let box_point = Box::new(origin());
    let box_rect = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    println!(
        "box_point {} bytes on the stack",
        mem::size_of_val(&box_point)
    );
    println!(
        "box_rect {} bytes on the stack",
        mem::size_of_val(&box_rect)
    );

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());
    println!(
        "box_in_a_box {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    let heap_point = *box_point;
    println!("{} bytes on the stack", mem::size_of_val(&heap_point));
}
