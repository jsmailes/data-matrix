use terminal_size::{Width, Height, terminal_size};

use console_engine::{ConsoleEngine, pixel, Color, KeyCode};

struct Line {
    text: String,
    trails: Vec<String>,
    x: u32,
    y: u32,
}

fn update(height: u32, mut lines: Vec<Line>) -> Vec<Line> {
    for line in &mut lines {
        line.y += 1;
    }

    // remove all elements which have fully moved beyond the bottom of the screen
    /*let drained: Vec<Line> = lines
        .drain(..)
        .filter(|line| {
            !line.trails.iter().any(|trail| i64::from(line.y) - (trail.len() as i64) < i64::from(height) )
        })
        .collect();*/

    lines
}

fn draw(lines: &Vec<Line>, engine: &mut ConsoleEngine){
    for line in lines {
        let text = &line.text[..];
        engine.print_fbg(line.x as i32, line.y as i32, text, Color::Green, Color::Reset);
    }
}

fn main() {
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line {
        text: String::from("500"),
        trails: vec![
            String::from("asdfasdf"),
            String::from("jklj"),
            String::from("qwertyuiop"),
        ],
        x: 15,
        y: 10,
    });

    let width: u32;
    let height: u32;

    if let Some((Width(w), Height(h))) = terminal_size() {
        width = u32::from(w);
        height = u32::from(h);
    } else {
        panic!("Unable to get terminal size");
    }

    const FPS: u32 = 3;

    let mut engine = ConsoleEngine::init(width, height, FPS).unwrap();


    loop {
        engine.wait_frame();
        engine.clear_screen();

        lines = update(height, lines);
        draw(&lines, &mut engine);

        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.draw();
    }
}