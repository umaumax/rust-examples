use std::env;
use std::error::Error;

fn read_feature_points(filename: String) -> Result<Vec<(u32, u32, f64)>, Box<dyn Error>> {
    type Record = (u32, u32, f64);
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_path(filename)?;

    let mut feature_points = Vec::with_capacity(128);

    for (_i, result) in rdr.deserialize().enumerate() {
        let record: Record = result?;
        feature_points.push(record);
    }
    Ok(feature_points)
}
fn main() {
    let mut args = env::args();
    args.next(); // skip command name

    let input = args.next().unwrap();
    let csv_filename = args.next().unwrap();
    let all_feature_points = read_feature_points(csv_filename).unwrap();

    let output = "out.png";
    let mut img = image::open(input).unwrap().to_rgba8();

    let (width, height) = img.dimensions();
    println!("img=({},{})", width, height);

    for fp in &all_feature_points {
        let (x, y, alpha) = (fp.0, fp.1, (fp.2 * 255.0) as u8);
        let color = image::Rgba([0u8, 0u8, 255u8, alpha]);
        img.put_pixel(x, y, color);
        println!("(x,y,a)=({},{},{})", x, y, alpha);
        let radius = 16;
        imageproc::drawing::draw_hollow_circle_mut(&mut img, (x as i32, y as i32), radius, color);
    }

    img.save(output).unwrap();
}
