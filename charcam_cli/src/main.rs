extern crate camera_capture;

use std::{io::Write, time::Duration};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use charcam::Voxel;

struct Resolution {
    w: u32,
    h: u32,
}

fn voxel_printer(voxels: Vec<Voxel>, width: usize) {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 1),).unwrap();
    voxels.iter().enumerate().for_each(|(i, voxel)| {
        if i % width == 0 {
            write!(
                stdout,
                "{}",
                termion::cursor::Goto(1, i as u16 / width as u16)
            )
            .unwrap();
        }
        write!(
            stdout,
            "{}{}",
            termion::color::Fg(termion::color::Rgb(voxel.r, voxel.g, voxel.b)),
            voxel.c
        )
        .unwrap();
    });
    stdout.lock().flush().unwrap();
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

    let ratio_width = (res.w as f32 / target_width as f32).ceil();
    let ratio_height = (res.h as f32 / target_height as f32).ceil();

    let final_width = (res.w as f32 / ratio_width).floor() as usize;
    // let final_height = res.h as usize / ratio_height;

    write!(stdout, "{}", termion::clear::All).unwrap();

    write!(
        stdout,
        "{}Ctr+c for exit",
        termion::cursor::Goto(1, terminal_height)
    )
    .unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    for image in cam {
        let matrix: Vec<Voxel> = image
            .pixels()
            .enumerate()
            .filter_map(|(i, value)| {
                // Coords on origin structure
                let y = i / res.w as usize;
                let x = i - (y * res.w as usize);

                if x % ratio_width as usize == 0 && y % ratio_height as usize == 0 {
                    return Some(value.data);
                }
                None
            })
            .map(|[r, g, b]| Voxel::new(r, g, b))
            .collect();

        voxel_printer(matrix, final_width);

        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Ctrl('c') => break,
                termion::event::Key::Char('q') => break,
                _ => {
                    // Do nothing
                }
            }
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    write!(
        stdout,
        "{}{}",
        termion::color::Fg(termion::color::White),
        termion::cursor::Show
    )
    .unwrap();
}
