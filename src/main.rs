use font_kit::{family_name::FamilyName::*, properties::Properties, source::SystemSource};

fn main() {
    for family in [Serif, SansSerif, Monospace, Cursive, Fantasy] {
        let font = SystemSource::new()
            .select_best_match(&[family], &Properties::new())
            .unwrap()
            .load()
            .unwrap()
            .full_name();
        println!("The default font path is: {:?}", font);
    }
}
