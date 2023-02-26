use std::env;
use std::path::Path;
fn main() {
    println!("img tool...");
    let image_path = env::args().skip(1).next().unwrap(); // 跳过第一个参数，文件名
    let path = Path::new(&image_path);
    let img = image::open(path).unwrap();
    let rotated = img.rotate90();
    rotated.save(path).unwrap(); // 保存为原路径
    println!("rotate90 success");
}
