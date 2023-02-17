#[derive(Debug)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Debug)]
struct Line {
    points: Vec<Point>
}

impl Line {
    fn validate(&self) -> bool {
        self.points.len() == 2
    }
    
    fn slope(&self) -> f64 {
        let p1 = &self.points[0];
        let p2 = &self.points[1];
        
        let s = (p2.y as f64 - p1.y as f64) / (p2.x as f64 - p1.x as f64);
        
        return s
    }
}

fn are_parallel(l1: &Line, l2: &Line) -> Option<bool> {
    if l1.validate() & l2.validate() {
        Some(false)
    } else {
        None
    }
}

fn intersect(l1: &Line, l2: &Line) -> Option<Point> {
    if l1.validate() & l2.validate() {
        Some(
            Point {
                x: 12,
                y: 15
            }
        )
    } else {
        None
    }
}

fn main() {
    let l1 = Line {
        points: vec![Point {x: 13, y: 2}, Point {x: 3, y: 4}]
    };
    
    let l2 = Line {
        points: vec![Point {x: 1, y: 2}, Point {x: 7, y: 8}]
    };
    
    let pi = intersect(&l1, &l2);
    
    println!("L1: {:?}", l1);
    println!("L1 validate: {:?}", l1.validate());
    println!("L1 slope: {:?}", l1.slope());
    println!("L1: {:?}", l1);
    println!("L2: {:?}", l2);
    println!("L1 L2 Parallel: {:?}", are_parallel(&l1, &l2));
    println!("Point Intersection: {:?}", pi);
}
