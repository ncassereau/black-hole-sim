use macroquad::prelude::*;

#[macroquad::main("Mon carré")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_rectangle(100.0, 100.0, 50.0, 50.0, RED);
        next_frame().await;
    }
}