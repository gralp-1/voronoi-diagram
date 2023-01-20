use config::{Config, ConfigError};
use raylib::prelude::*;
use crate::config::Settings;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub being_dragged: bool,
    pub color: Color,
}

impl Point {
    fn new(s: &Settings) -> Self {
        Self {
            x: (rand::random::<f32>() * s.width as f32) ,
            y: (rand::random::<f32>() * s.height as f32) ,
            being_dragged: false,
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
    pub fn new(s:&Settings) -> Self {
        let mut points = vec![];
        for _ in 0..s.points{
            points.push(Point::new(&s));
        }
        Self { points }
    }
    pub fn add_point(&mut self, (x, y): (f32, f32)) {
        self.points.push(Point {
            x,
            y,
            being_dragged: false,
            color: Color::new(
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
                255 as u8,
            ),
        });
    }
    pub fn nearest(self, (x, y): (f32, f32)) -> Point {
        let mut nearest = Point{
            x: 0.0,
            y: 0.0,
            being_dragged: false,
            color: Color::BLUE,
        };
        let mut min_distance = 0.0;
        for point in &self.points {
            let distance = (((point.x - x).powf(2.0) + (point.y - y).powf(2.0)) as f32).sqrt();
            if distance < min_distance || min_distance == 0.0 {
                min_distance = distance;
                nearest = *point;
            }
        }
        nearest
    }
}
