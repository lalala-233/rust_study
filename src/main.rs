use font_kit::source::SystemSource;

fn main() {
    let font = SystemSource::new().all_fonts();
    println!("The default font path is: {:#?}", font);
}
