use terminal_size::{Width, Height, terminal_size};

use console_engine::{ConsoleEngine, Color, KeyCode, KeyModifiers};

use rand::Rng;
use rand::prelude::ThreadRng;
use rand::distributions::{Distribution, Uniform};

use std::io;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::time::Duration;


struct Line {
    text: String,
    trails: Vec<u32>,
    x: u32,
    y: u32,
}

// update lines, remove outdated lines
fn update_lines(height: u32, mut lines: Vec<Line>) -> Vec<Line> {
    // update line positions
    for line in &mut lines {
        line.y += 1;
    }

    // remove all elements which have fully moved beyond the bottom of the screen
    lines.retain(|line| {
        !line.trails.iter().all(|trail| i64::from(line.y) - (*trail as i64) > i64::from(height) )
    });

    lines
}

// update trail background, changing each cell with probability chance
fn update_trails(mut trails: Vec<Vec<char>>, chance: f64, rng: &mut ThreadRng, char_distribution: &Uniform<char>) -> Vec<Vec<char>> {
    for i in 0..trails.len() {
        for j in 0..trails[i].len() {
            let c: f64 = rng.gen_range(0.0..1.0);
            if c < chance {
                trails[i][j] = char_distribution.sample(rng);
            }
        }
    }

    trails
}

fn add_line(mut lines: Vec<Line>, text: String, width: u32, trail_length: u32, rng: &mut ThreadRng) -> Vec<Line> {
    let mut trails: Vec<u32> = Vec::new();
    for _ in 0..text.len() {
        trails.push(rng.gen_range(1..trail_length));
    }
    let w = width - std::cmp::min(text.len() as u32, width-1);
    let x: u32 = rng.gen_range(0..w);
    lines.push(Line {
        text,
        trails,
        x,
        y: 0,
    });

    lines
}

// draw all lines and associated trails
fn draw(lines: &Vec<Line>, trails: &Vec<Vec<char>>, engine: &mut ConsoleEngine, line_color: Color, trail_color: Color){
    for line in lines {
        // draw main text
        let text = &line.text[..];
        engine.print_fbg(line.x as i32, line.y as i32, text, line_color, Color::Reset);

        // draw trails
        let mut trail_buffer = [0; 4];
        for (i, l) in line.trails.iter().enumerate() {
            for j in 0..*l {
                match trails.get(line.x as usize + i) {
                    Some(cs) => {
                        match cs.get(line.y as usize - std::cmp::min(j as usize + 1, line.y as usize)) {
                            Some(c) => {
                                let s = c.encode_utf8(&mut trail_buffer);
                                engine.print_fbg(line.x as i32 + i as i32, line.y as i32 - j as i32 - 1, s, trail_color, Color::Reset);
                            },
                            None => ()
                        }
                    },
                    None => ()
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut rng = rand::thread_rng();
    //let char_distribution = Uniform::from((166 as char)..(217 as char));
    let char_distribution = Uniform::from((0x21 as char)..(0x7e as char));

    let mut lines: Vec<Line> = Vec::new();
    let mut lines_bg: Vec<Line> = Vec::new();

    let width: u32;
    let height: u32;

    if let Some((Width(w), Height(h))) = terminal_size() {
        width = u32::from(w);
        height = u32::from(h);
    } else {
        panic!("Unable to get terminal size");
    }

    let mut trails: Vec<Vec<char>> = Vec::new();
    for _ in 0..width {
        let mut ts: Vec<char> = Vec::new();
        for _ in 0..height {
            ts.push(char_distribution.sample(&mut rng));
        }
        trails.push(ts);
    }

    const FPS: u32 = 15;
    let mut engine = ConsoleEngine::init(width, height, FPS).unwrap();

    let (tx_stdin, rx_stdin): (Sender<String>, Receiver<String>) = mpsc::channel();

    let _handle = thread::spawn(move || {
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    if n == 0 {
                        // EOF reached
                        break;
                    }
                    if n > 1 {
                        // Remove newline character
                        tx_stdin.send(input[..input.len()-1].to_string()).unwrap();
                    }
                },
                Err(_) => ()
            }
        }
    });

    let timeout_duration = Duration::from_millis(1);

    let mut trail_buffer = [0; 4];

    loop {
        engine.wait_frame();
        engine.clear_screen();

        lines = update_lines(height, lines);
        trails = update_trails(trails, 0.05, &mut rng, &char_distribution);

        match rx_stdin.recv_timeout(timeout_duration) {
            Ok(input) => {
                lines = add_line(lines, input, width, 16, &mut rng);
            },
            Err(_) => ()
        }

        lines_bg = update_lines(height, lines_bg);
        let c = char_distribution.sample(&mut rng).encode_utf8(&mut trail_buffer);
        lines_bg = add_line(lines_bg, c.to_string(), width, 16, &mut rng);

        draw(&lines_bg, &trails, &mut engine, Color::Grey, Color::Grey);
        draw(&lines, &trails, &mut engine, Color::White, Color::Green);

        if engine.is_key_pressed(KeyCode::Char('q')) || engine.is_key_pressed_with_modifier(KeyCode::Char('c'), KeyModifiers::CONTROL) {
            break;
        }

        engine.draw();
    }

    Ok(())
}
