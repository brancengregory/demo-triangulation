use demo_triangulation_core as tri;
use text_io::read;

fn main() {
    print!("Enter the coordinates of the first line (x1 y1 x2 y2): ");
    let l1 = tri::Line::new(
        tri::Point {x: read!(), y: read!()},
        tri::Point {x: read!(), y: read!()}
    ).unwrap();
    
    print!("Enter the coordinates of the second line (x1 y1 x2 y2): ");
    let l2 = tri::Line::new(
        tri::Point {x: read!(), y: read!()},
        tri::Point {x: read!(), y: read!()}
    ).unwrap();
    
    let pi = tri::intersect(&l1, &l2).unwrap();

    println!("L1: {:?}", l1);
    println!("L2: {:?}", l2);
    println!("Point Intersection: {:?}", pi);
}
