use image::{GenericImage, ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::{
    draw_antialiased_line_segment_mut, draw_filled_circle_mut, draw_filled_rect_mut,
};
use imageproc::pixelops::interpolate;
use imageproc::rect::Rect;
use indicatif::ProgressBar;
use itermore::IterMore;
use std::io::{stdin, Read};
use std::sync::Arc;
use std::thread::spawn;
use tsp_sa::point::*;

fn draw_points(points: Arc<Vec<Point>>) {
    let maxx = points.iter().map(|p| p.x).max().unwrap();
    let maxy = points.iter().map(|p| p.y).max().unwrap();
    let mut base_img = ImageBuffer::new((maxx * 10 + 20) as u32, (maxy * 10 + 20) as u32);
    let rect = Rect::at(0, 0).of_size(base_img.width(), base_img.height());
    draw_filled_rect_mut(&mut base_img, rect, Rgb([255, 255, 255]));
    for p in &*points {
        draw_filled_circle_mut(&mut base_img, p.to_coord(), 5, Rgb([255, 0, 0]));
    }
    base_img.save("base.png").unwrap();
    let file_u8 = stdin().bytes().map(|ch| ch.unwrap()).collect::<Vec<u8>>();
    let file = Arc::new(std::str::from_utf8(&file_u8).unwrap().to_string());
    let mut threads = Vec::<std::thread::JoinHandle<()>>::new();
    let base_img = Arc::new(base_img);
    let bar = ProgressBar::new(file.lines().count() as u64);
    for x in 0..16 {
        let file = file.clone();
        let base_img = base_img.clone();
        let points = points.clone();
        let bar = bar.clone();
        threads.push(spawn(move || {
            let mut lines: Box<dyn Iterator<Item = &str>> = Box::new(file.lines().skip(x));
            let mut i = x;
            while let Some(line) = lines.next() {
                let mut img: RgbImage = ImageBuffer::new(base_img.width(), base_img.height());
                img.copy_from(&*base_img, 0, 0).unwrap();
                // let point: Vec<usize> =
                //     .collect();
                for [prev, next] in line
                    .split(" ")
                    .filter_map(|s| s.parse::<usize>().ok())
                    .windows()
                {
                    draw_antialiased_line_segment_mut(
                        &mut img,
                        points[prev].to_coord(),
                        points[next].to_coord(),
                        Rgb([0, 0, 0]),
                        interpolate,
                    );
                }
                img.save(format!("{}.png", i)).unwrap();
                i += 16;
                lines = Box::new(lines.skip(15));
                bar.inc(1);
            }
        }))
    }
    for thread in threads {
        thread.join().unwrap();
    }
    bar.finish();
}

fn main() {
    let points = read_points(false);
    draw_points(Arc::new(points));
}
