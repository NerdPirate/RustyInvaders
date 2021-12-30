
use crate::engine::Bitmap;

mod engine;


fn main() {
    let bm = Bitmap::build_from_file("./src/test1.json");
    println!("Bitmap:\n{}", bm);
}
