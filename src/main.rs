pub mod vector3;

use vector3::{Color, write_color};
fn main(){
    // println!("working");
    const IMAGE_HEIGHT: i32 = 256;
    const IMAGE_WIDTH: i32 = 256;

    println!("P3\n{IMAGE_HEIGHT} {IMAGE_WIDTH}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH{
            let pixel_color = Color::new( 
                (i as f64) / (IMAGE_WIDTH as f64 - 1.0),
                (j as f64) / (IMAGE_HEIGHT as f64 - 1.0),
                0.25);
            write_color(pixel_color);
        }
    }
}
