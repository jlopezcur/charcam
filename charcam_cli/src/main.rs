use nokhwa::{Camera, CameraFormat, FrameFormat};
use std::{io::Write, time::Duration};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use charcam::Voxel;

struct Size {
    w: u32,
    h: u32,
}

fn voxel_printer(voxels: Vec<Voxel>, white_mode: bool) {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Goto(1, 1),).unwrap();
    voxels.iter().for_each(|voxel| {
        write!(
            stdout,
            "{}{}{}",
            termion::cursor::Goto(voxel.x as u16, voxel.y as u16 + 1),
            if white_mode {
                termion::color::Fg(termion::color::Rgb(255, 255, 255))
            } else {
                termion::color::Fg(termion::color::Rgb(voxel.r, voxel.g, voxel.b))
            },
            voxel.c
        )
        .unwrap();
    });
    stdout.lock().flush().unwrap();
}

fn main() {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    let mut white_mode = false;

    let origin = Size { w: 320, h: 180 };
    let mut camera = Camera::new(
        0,
        Some(CameraFormat::new_from(
            origin.w,
            origin.h,
            FrameFormat::MJPEG,
            30,
        )),
    )
    .unwrap();
    camera.open_stream().unwrap();

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide).unwrap();

    loop {
        if let Ok(frame) = camera.frame() {
            let (terminal_width, terminal_height) = termion::terminal_size().ok().unwrap();
            let target = Size {
                w: terminal_width as u32,
                h: terminal_height as u32,
            };

            let img = image::imageops::resize(&frame, target.w, target.h, image::imageops::Nearest);
            let voxels: Vec<Voxel> = img
                .enumerate_pixels()
                .map(|(x, y, pixel)| (x, y, pixel.0))
                .map(|(x, y, [r, g, b])| Voxel::new(x, y, r, g, b))
                .collect();

            voxel_printer(voxels, white_mode);

            let input = stdin.next();
            if let Some(Ok(key)) = input {
                match key {
                    termion::event::Key::Ctrl('c') => break,
                    termion::event::Key::Char('q') => break,
                    termion::event::Key::Char('w') => {
                        white_mode = !white_mode;
                    }
                    _ => {}
                }
            }

            std::thread::sleep(Duration::from_millis(1));
        }
    }

    write!(
        stdout,
        "{}{}",
        termion::color::Fg(termion::color::White),
        termion::cursor::Show
    )
    .unwrap();
}
