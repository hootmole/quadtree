use std::io::copy;

use rand::Rng;




fn calculate_force_between_points(point1: &Point, point2: (f64, f64, f32)) -> (f64, f64) {
    let G: f64 = 6.6743e-11; // Gravitational constant
    let dx: f64 = point2.0 - point1.position.0;
    let dy: f64 = point2.1 - point1.position.1;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance == 0.0 {
        (0.0, 0.0)
    } 
    else {
        let force = G * (point1.mass * point2.2 as f32) as f64 / (distance * distance);
        let fx = force * dx / distance;
        let fy = force * dy / distance;
        (fx, fy)
    }

    
}

fn calculate_distance(p1: &Point, p2: &Point) -> f64 {
    let dx = p2.position.0 - p1.position.0;
    let dy = p2.position.1 - p1.position.1;

    (dx * dx + dy * dy).sqrt()
}



#[derive(Debug, Clone, Copy)]
struct Point {
    position: (f64, f64),
    mass: f32,
    velocity: (f32, f32),
    acceleration: (f32, f32),

}

impl Point {
    fn new(position: (f64, f64)) -> Self {
        Point { position,  mass: 1.0, velocity: (0.0, 0.0), acceleration: (0.0, 0.0)}
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

    fn get_center_of_mass(&self) -> (f64, f64, f32) {
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
                // let (child_x, child_y, _) = child.get_center_of_mass();
                // sum_x += child_x * child.get_total_mass() as f64;
                // sum_y += child_y * child.get_total_mass() as f64;
                // total_mass += child.get_total_mass();

                let (child_x, child_y, mass) = child.get_center_of_mass();
                sum_x += child_x * mass as f64;
                sum_y += child_y * mass as f64;
                total_mass += child.get_total_mass();
            }
        }
    
        if total_mass > 0.0 {
            (sum_x / total_mass as f64, sum_y / total_mass as f64, total_mass)
        } else {
            (0.0, 0.0, 0.0) // Handle case where no points exist
        }
    }

    fn get_total_mass(&self) -> f32 {
        let mut total_mass: f32 = 0.0;
    
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

    fn get_pointer_array(&self) -> Vec<&Point> {
        let mut result = Vec::new();
    
        if let Some(point) = &self.point { // Get a reference to the point
            result.push(point);
        }
    
        if let Some(ref children) = self.children {
            for child in children.iter() {
                result.extend(child.get_pointer_array()); // Recursively collect pointers
            }
        }
        result
    }

    fn compute_force_sum(&self, active_point: &Point) -> (f64, f64) {
        let mut force_vector = (0.0, 0.0);
        // println!("force compute");

        if let Some(ref point) = self.point {
            let s = calculate_distance(active_point, point);
            let d = self.boundary.width;

            if s / d < self.delta as f64 && s != d || 1 == 1{ // aproximation
                let p2 = self.get_center_of_mass();
                let force = calculate_force_between_points(active_point, p2);
                force_vector.0 += force.0;
                force_vector.1 += force.1;
                println!("aprox done");
            }
            else {
                let force = calculate_force_between_points(active_point, (point.position.0, point.position.1, point.mass));
                force_vector.0 += force.0;
                force_vector.1 += force.1;
            }
        }
    
        if let Some(ref children) = self.children {
            for child in children.iter() {
                let force = child.compute_force_sum(active_point);
                force_vector.0 += force.0;
                force_vector.1 += force.1;
            }
        }
        force_vector
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

    for i in 1..20000 {
        let x: f64 = rng.gen::<f64>() * 100.0;
        let y: f64 = rng.gen::<f64>() * 100.0;

        let mut point: Point = Point::new((x, y));
        point.acceleration = (i as f32, i as f32);
        quadtree.insert(point);
        
    };
    let mut p = Point::new((5.0, 5.0));
    p.mass = 60.0;
    quadtree.insert(p);

    let result = quadtree.get_pointer_array();

    let force = quadtree.compute_force_sum(result[1]);
    

    println!("{:?}", force);
    println!("{:?}", result[1000]);

    //println!("points: {:?}", result);
}


// 1) vygenerovat array s body ✔
// 2) vytvorit quadtree a vlozit tam vygenerovane body ✔
// 3) iterovat pres vsechny body a vypocitat acceleration vektor kazdeho bodu podle barnes hut algoritmu
// 4) nacist body do noveho arraye
// 5) vypocitat novou pozici bodu z arraye
// 6) vytvorit peknej obrazek
// 7) GOTO 2