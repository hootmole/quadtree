use std::io::copy;

use rand::Rng;


#[derive(Debug, Clone, Copy)]
struct Point {
    position: (f64, f64),
    mass: f32,
    velocity: (f32, f32),
    acceleration: (f32, f32),
    is_scanned: bool,

}

impl Point {
    fn new(position: (f64, f64)) -> Self {
        Point { position,  mass: 1.0, velocity: (0.0, 0.0), acceleration: (0.0, 0.0), is_scanned: 1 != 0}
    }
    // fn change(&mut self, position: (f64, f64), velocity: (f64, f64)) {
    //     self.position
    // }
}

#[derive(Debug)]
struct Quadtree {
    boundary: Rectangle,
    point: Option<Point>,
    children: Option<Box<[Quadtree; 4]>>,
    delta: f32,
}

impl Quadtree {
    fn new(boundary: Rectangle) -> Self {
        Quadtree {
            boundary,
            point: None,
            children: None,
            delta: 0.5,
        }
    }

    fn insert(&mut self, point: Point) {
        if !self.boundary.contains(&point) {
            return;
        }

        if self.point.is_none() {
            self.point = Some(point);
        } 
        else {
            if self.children.is_none() {
                self.subdivide();
            }

            if let Some(ref mut children) = self.children {
                for child in children.iter_mut() {
                    child.insert(point);
                }
            }
        }
    }

    fn subdivide(&mut self) {
        let Rectangle { x, y, width, height } = self.boundary;
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let nw_boundary = Rectangle::new(x, y, half_width, half_height);
        let ne_boundary = Rectangle::new(x + half_width, y, half_width, half_height);
        let sw_boundary = Rectangle::new(x, y + half_height, half_width, half_height);
        let se_boundary = Rectangle::new(x + half_width, y + half_height, half_width, half_height);

        self.children = Some(Box::new([
            Quadtree::new(nw_boundary),
            Quadtree::new(ne_boundary),
            Quadtree::new(sw_boundary),
            Quadtree::new(se_boundary),
        ]));
    }

    fn query(&self, range: &Rectangle) -> Vec<Point> {
        let mut result = Vec::new();

        if !self.boundary.intersects(range) {
            return result;
        }

        if let Some(point) = self.point {
            if range.contains(&point) {
                result.push(point);
            }
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                result.extend(child.query(range));
            }
        }

        result
    }

    fn get_points(&self) -> Vec<Point>{
        let mut result = Vec::new();

        if let Some(point) = self.point {
            result.push(point)
        }
        if let Some(ref children) = self.children {
            for child in children.iter() {
                result.extend(child.get_points());
            }
        }
        result
    }

    fn get_center_of_mass(&self) -> (f64, f64) {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut total_mass = 0.0;
    
        if let Some(point) = self.point {
            sum_x += point.position.0 * point.mass as f64;
            sum_y += point.position.1 * point.mass as f64;
            total_mass += point.mass;
        }
    
        if let Some(ref children) = self.children {
            for child in children.iter() {
                let (child_x, child_y) = child.get_center_of_mass();
                sum_x += child_x * child.get_total_mass() as f64;
                sum_y += child_y * child.get_total_mass() as f64;
                total_mass += child.get_total_mass();
            }
        }
    
        if total_mass > 0.0 {
            (sum_x / total_mass as f64, sum_y / total_mass as f64)
        } else {
            (0.0, 0.0) // Handle case where no points exist
        }
    }

    fn get_total_mass(&self) -> f32 {
        let mut total_mass = 0.0;
    
        if let Some(point) = self.point {
            total_mass += point.mass;
        }
    
        if let Some(ref children) = self.children {
            for child in children.iter() {
                total_mass += child.get_total_mass();
            }
        }
    
        total_mass
    }
    
    fn compute(&self) {
        ;
    }
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rectangle { x, y, width, height }
    }

    fn contains(&self, point: &Point) -> bool {
        point.position.0 >= self.x && point.position.0 <= self.x + self.width &&
        point.position.1 >= self.y && point.position.1 <= self.y + self.height
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }
} 

fn main() {
    // Example usage
    let mut quadtree = Quadtree::new(Rectangle::new(0.0, 0.0, 100.0, 100.0));

    // Insert points into the quadtree
    // quadtree.insert(Point::new((25.0, 25.0)));
    // quadtree.insert(Point::new((750.0, 75.0)));

    // Query points within a range
    // let query_range = Rectangle::new(0.0, 0.0, 100.0, 100.0);

    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let mut points: Vec<&Point> = Vec::new();

    for i in 1..700 {
        let x: f64 = rng.gen::<f64>() * 100.0;
        let y: f64 = rng.gen::<f64>() * 100.0;

        let mut point: Point = Point::new((x, y));
        point.acceleration = (i as f32, i as f32);
        quadtree.insert(point);
        
    };

    let result = quadtree.get_points();

    let xd = quadtree.get_total_mass();
    

    println!("{:?}", xd)


    //println!("points: {:?}", result);
}


// 1) vygenerovat array s body ✔
// 2) vytvorit quadtree a vlozit tam vygenerovane body ✔
// 3) iterovat pres vsechny body a vypocitat acceleration vektor kazdeho bodu podle barnes hut algoritmu
// 4) nacist body do noveho arraye
// 5) vypocitat novou pozici bodu z arraye
// 6) vytvorit peknej obrazek
// 7) GOTO 2