extern crate camera_capture;

use std::{io::Write, time::Duration};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn brightness(r: f32, g: f32, b: f32) -> f32 {
    (0.299 * r + 0.587 * g + 0.114 * b) / 255.0
}

fn density_char(factor: &f32) -> char {
    let density = "Ñ@#W$9876543210?!abc;:+=-,._ ";
    let index = (factor * density.len() as f32).floor();
    density.chars().nth(index as usize).unwrap()
}

struct Resolution {
    w: u32,
    h: u32,
}

fn main() {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    let (terminal_width, terminal_height) = termion::terminal_size().ok().unwrap();
    let cam = camera_capture::create(0).unwrap();
    let res = Resolution { w: 320, h: 180 };
    let cam = cam
        .fps(30.0)
        .unwrap()
        .resolution(res.w, res.h)
        .unwrap()
        .start()
        .unwrap();

    let target_width = terminal_width as usize;
    let target_height = terminal_height as usize;

    let ratio_width = (res.w as f32 / target_width as f32).ceil() as usize;
    let ratio_height = (res.h as f32 / target_height as f32).ceil() as usize;

    let final_width = res.w as usize / ratio_width;

    write!(stdout, "{}", termion::clear::All).unwrap();

    write!(
        stdout,
        "{}Ctr+c for exit",
        termion::cursor::Goto(1, terminal_height)
    )
    .unwrap();

    for image in cam {
        let bright_matrix = image.pixels().map(|pixel| {
            let [r, g, b] = pixel.data;
            1.0 - brightness(r as f32, g as f32, b as f32)
        });
        let mut matrix = vec![];
        for (i, voxel) in bright_matrix.enumerate() {
            // Coords on origin structure
            let y = i / res.w as usize;
            let x = i - (y * res.w as usize);

            if x % ratio_width == 0 && y % ratio_height == 0 {
                matrix.push(voxel);
            }
        }

        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        for (i, voxel) in matrix.iter().enumerate() {
            if i % final_width == 0 {
                write!(
                    stdout,
                    "{}",
                    termion::cursor::Goto(1, i as u16 / final_width as u16)
                )
                .unwrap();
            }
            write!(stdout, "{}", density_char(voxel)).unwrap();
        }
        stdout.lock().flush().unwrap();
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Ctrl('c') => break,
                _ => {
                    // Do nothing
                }
            }
        }

        std::thread::sleep(Duration::from_millis(1));
    }
}
