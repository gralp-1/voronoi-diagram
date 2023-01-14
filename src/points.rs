
use raylib::prelude::*;

const POINTS: i32 = 10;
const HEIGHT: i32 = 450;
const WIDTH: i32 = 800;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Point {
    fn new() -> Self {
        Self {
            x: (rand::random::<f32>() * WIDTH as f32) as i32,
            y: (rand::random::<f32>() * HEIGHT as f32) as i32,
            // random color
            color: Color::new(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
                255 as u8,
            ),
            
        }
    }
}

#[derive(Debug, Clone)]
pub struct Points {
    pub points: Vec<Point>,
}

impl Points {
    pub fn new() -> Self {
        let mut points = vec![];
        for _ in 0..POINTS {
            points.push(Point::new());
        }
        Self { points }
    }
    pub fn add_point(&mut self, (x, y): (i32, i32)) {
        self.points.push(Point {
            x,
            y,
            color: Color::new(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
                255 as u8,
            ),
        });
    }
    pub fn nearest(self, (x, y): (i32, i32)) -> Point {
        let mut nearest = Point{
            x: 0,
            y: 0,
            color: Color::BLUE,
        };
        let mut min_distance = 0.0;
        for point in &self.points {
            let distance = (((point.x - x).pow(2) + (point.y - y).pow(2)) as f32).sqrt();
            if distance < min_distance || min_distance == 0.0 {
                min_distance = distance;
                nearest = *point;
            }
        }
        nearest
    }
}
