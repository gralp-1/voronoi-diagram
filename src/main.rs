
use rand::Rng;
use raylib::prelude::*;
mod points;
use points::{Points};
const HEIGHT: i32 = 450;
const WIDTH: i32 = 800;




fn main() {
    // initialise a raylib window
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("voronoi diagram")
        .resizable()
        .build();


    let mut points = Points::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        // if the user clicks, add a new point at the mouse position
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            let mouse = d.get_mouse_position();
            points.add_point((mouse.x as i32, mouse.y as i32));
        }
        // if they press r, reset the points
        if d.is_key_pressed(KeyboardKey::KEY_R) {
            points = Points::new();
        }

        // iterate over every pixel in the window
        for x in 0..d.get_screen_width(){
            for y in 0..d.get_screen_height() {
                // find the nearest point to the current pixel
                let nearest = points.clone().nearest((x, y));
                // draw the pixel with the color of the nearest point
                d.draw_pixel(x, y, nearest.color);
            }
        }




        // draw the voronoi points as black dots with radius which scales as the mouse gets closer
        for point in &points.points {
            let mouse = d.get_mouse_position();
            let t = 220.0;
            let z = 0.2;
            let dist = (((point.x as f32 - mouse.x).powf(2.0) + (point.y as f32 - mouse.y).powf(2.0)) as f32).sqrt();
            let radius = clamp(t/(10.0 + 1.1_f32.powf(z*dist)), 5.0, 15.0);
            d.draw_circle(point.x, point.y, radius , Color::BEIGE);
        }

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