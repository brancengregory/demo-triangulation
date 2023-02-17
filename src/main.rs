#[derive(Debug)]
#[derive(PartialEq)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Debug)]
struct Line {
    points: Vec<Point>
}

impl Line {
    fn new(p1: Point, p2: Point) -> Result<Line, String> {
        if p1 == p2 {
            Err("Points are the same. You must supply two unique points".to_string())
        } else {
            Ok(Line {
                points:  vec![p1, p2]
            })
        }
    }
    
    fn slope(&self) -> f64 {
        let p1 = &self.points[0];
        let p2 = &self.points[1];
        
        let s = (p2.y as f64 - p1.y as f64) / (p2.x as f64 - p1.x as f64);
        
        return s
    }

    fn y_intercept(&self) -> f64 {
        let p1 = &self.points[0];
        
        let s = self.slope();
        
        let y = p1.y as f64 - s * p1.x as f64;
        
        return y
    }
}

fn are_parallel(l1: &Line, l2: &Line) -> bool {
    // Use the slope and y-intercept to determine if the lines are parallel
    let s1 = l1.slope();
    let s2 = l2.slope();

    let y1 = l1.y_intercept();
    let y2 = l2.y_intercept();

    if s1 == s2 && y1 == y2 {
        return true
    } else {
        return false
    }
}

fn intersect(l1: &Line, l2: &Line) -> Option<Point> {
    // If the lines are parallel, they will never intersect
    if are_parallel(l1, l2) {
        return None
    } else {
        // Use the slope and y-intercept to determine the point of intersection
        let s1 = l1.slope();
        let s2 = l2.slope();

        let y1 = l1.y_intercept();
        let y2 = l2.y_intercept();

        let x = (y2 - y1) / (s1 - s2);
        let y = s1 * x + y1;

        return Some(Point {x: x.round() as i64, y: y.round() as i64})
    }
}

fn main() {
    let l1 = Line::new(
        Point {x: -282, y: 107},
        Point {x: -275, y: 118}
    ).unwrap();
    
    let l2 = Line::new(
        Point {x: -240, y: 39},
        Point {x: -233, y: 51}
    ).unwrap();
    
    let pi = intersect(&l1, &l2).unwrap();
    
    println!("L1: {:?}", l1);
    println!("L1 slope: {:?}", l1.slope());
    println!("L1: {:?}", l1);
    println!("L2: {:?}", l2);
    println!("L1 L2 Parallel: {:?}", are_parallel(&l1, &l2));
    println!("Point Intersection: {:?}", pi);
}
