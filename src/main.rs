use std::env;
use std::process;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct ColorBlock {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: [u8; 4],
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <image.png>", args[0]);
        process::exit(1);
    }

    let path = &args[1];
    let img = image::open(path).expect("Failed to open image");
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();

    let mut visited = vec![vec![false; width as usize]; height as usize];
    let mut blocks = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if visited[y as usize][x as usize] {
                continue;
            }

            let pixel = img.get_pixel(x, y).0;
            if pixel[3] == 0 {
                continue; // Skip transparent
            }

            // Try to grow a rectangle
            let mut w = 1;
            while x + w < width
                && img.get_pixel(x + w, y).0 == pixel
                && !visited[y as usize][(x + w) as usize]
            {
                w += 1;
            }

            let mut h = 1;
            'outer: while y + h < height {
                for dx in 0..w {
                    if img.get_pixel(x + dx, y + h).0 != pixel
                        || visited[(y + h) as usize][(x + dx) as usize]
                    {
                        break 'outer;
                    }
                }
                h += 1;
            }

            for dy in 0..h {
                for dx in 0..w {
                    visited[(y + dy) as usize][(x + dx) as usize] = true;
                }
            }

            blocks.push(ColorBlock {
                x,
                y,
                width: w,
                height: h,
                color: pixel,
            });
        }
    }

    // Print the Macroquad draw function
    println!("pub fn draw_exported_image(x_offset: f32, y_offset: f32, pixel_size: f32) {{");
    for block in blocks {
        println!(
            "    draw_rectangle(x_offset + {}.0 * pixel_size, y_offset + {}.0 * pixel_size, {}.0 * pixel_size, {}.0 * pixel_size, Color::from_rgba({}, {}, {}, {}));",
            block.x, block.y, block.width, block.height,
            block.color[0], block.color[1], block.color[2], block.color[3]
        );
    }
    println!("}}");
}
