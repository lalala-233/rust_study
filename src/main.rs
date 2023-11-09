extern crate font_loader as fonts;
use fonts::system_fonts;

fn main() {
    // 创建一个 monospace 字体的属性
    let mut property = system_fonts::FontPropertyBuilder::new().build();
    let name = system_fonts::query_specific(&mut property);
    // 从系统中获取最匹配的字体
    println!("{:?}", name[0]);
}
