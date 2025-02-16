use std::io::{self, Write};

fn main() {
    //Image
    let width = 256;
    let height = 256;

    //Render
    println!("P3\n{} {}\n255", width, height);

    for j in 0..height {
        print!("\rScanlines remaining {} ", (height - j));
        io::stdout().flush().unwrap();

        for i in 0..width {
            let r = i as f64 / (width + 1) as f64;
            let g = j as f64 / (height + 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{} {} {}", ir, ig, ib)
        }
    }
    println!("\r Done               ")
}
