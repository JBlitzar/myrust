use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dan game".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        let t = get_time() as f32;

        let x = screen_width() * 0.5 + (t.sin() * 200.0);

        let y = screen_height() * 0.5;

        draw_circle(x, y, 40.0, RED);

        draw_text("Hello world", 20.0, 40.0, 30.0, WHITE);

        next_frame().await;
    }
}
