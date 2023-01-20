
use raylib::prelude::*;
mod points;
mod config;

use points::{Points, Point};
use crate::config::Settings;

fn main() {
    // initialise a raylib window
    let s = Settings::new().unwrap();
    let (mut rl, thread) = raylib::init()
        .size(s.width, s.height)
        .title("voronoi diagram")
        .resizable()
        .build();


    let mut points = Points::new(&s);
    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);


        for x in 0..d.get_screen_width(){
            for y in 0..d.get_screen_height() {
                // find the nearest point to the current pixel
                let nearest_point = nearest(&points.points, x as f32, y as f32);
                // draw the pixel with the color of the nearest point
                let colour = noise(nearest_point.color, &s);
                d.draw_pixel(x, y, colour);
            }
        }

        control_handler(&mut points, &mut d);

        if d.is_key_pressed(KeyboardKey::KEY_R) {
            points = Points::new(&s);
        }

        if s.show_fps {
            d.draw_fps(10, 10);
        }
    }
}
fn noise(colour: Color, s: &Settings) -> Color {
    let r = colour.r as f32 + (rand::random::<f32>() * s.noise_amount);// - (amount / 2.0);
    let g = colour.g as f32 + (rand::random::<f32>() * s.noise_amount);// - (amount / 2.0);
    let b = colour.b as f32 + (rand::random::<f32>() * s.noise_amount);// - (amount / 2.0);
    Color::new(r as u8, g as u8, b as u8, 255)
}
fn nearest(points: &Vec<Point>, x: f32, y: f32) -> Point {
    let mut nearest = points[0];
    let mut dist = ((nearest.x - x).powf(2.0) + (nearest.y - y).powf(2.0)).sqrt();
    for point in points{
        let new_dist = ((point.x - x).powf(2.0) + (point.y - y).powf(2.0)).sqrt();
        if new_dist < dist {
            dist = new_dist;
            nearest = *point;
        }
    }
    nearest
}

fn control_handler(mut points: &mut Points, d: &mut RaylibDrawHandle){
    if d.is_key_pressed(KeyboardKey::KEY_D) {
        let mouse = d.get_mouse_position();
        let nearest = points.clone().nearest((mouse.x, mouse.y));
        // make sure it doesn't delete the last point
        if points.points.len() > 1 {
            points.points.retain(|&x| x != nearest);
        }
    }
    if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        let mouse = d.get_mouse_position();
        points.add_point((mouse.x, mouse.y));
    }
    // if they press r, reset the points
    for mut point in &mut points.points {

        if d.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
            let mouse = d.get_mouse_position();
            if (point.x - mouse.x).abs() < 10.0 && (point.y - mouse.y).abs() < 10.0 {
                point.being_dragged = true;
            }
        }

        if d.is_mouse_button_released(MouseButton::MOUSE_RIGHT_BUTTON) {
            point.being_dragged = false;
        }
        if d.is_mouse_button_down(MouseButton::MOUSE_RIGHT_BUTTON) {
            if point.being_dragged {
                let mouse = d.get_mouse_position();
                point.x = mouse.x;
                point.y = mouse.y;
            }
        }

        // Draw points
        let mouse = d.get_mouse_position();
        let t = 220.0;
        let z = 0.2;
        let dist = (((point.x as f32 - mouse.x).powf(2.0) + (point.y as f32 - mouse.y).powf(2.0)) as f32).sqrt();
        let radius = clamp(t/(10.0 + 1.1_f32.powf(z*dist)), 5.0, 15.0);
        d.draw_circle(point.x as i32, point.y as i32, radius , Color::BEIGE);
    }

}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}