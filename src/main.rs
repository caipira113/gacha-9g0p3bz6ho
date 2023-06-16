use rand::Rng;
use crossterm::style::{Color, style, Stylize};

const GACHA_ITEMS: [(f64, &str, Color); 4] = [
    (0.01, "SSR", Color::Magenta),
    (0.10, "Super Rare", Color::Red),
    (0.30, "Rare", Color::Yellow),
    (1.00, "Common", Color::White),
];

fn main() {
    let mut rng = rand::thread_rng();
    let gacha_result = do_gacha(&GACHA_ITEMS, &mut rng);
    println!("이번의 가챠 결과는...!\n{}", gacha_result);
}

fn do_gacha(items: &[(f64, &str, Color)], rng: &mut impl Rng) -> String {
    let rand_num: f64 = rng.gen();

    let (rate, item, color) = *items
        .iter()
        .find(|(rate, _, _)| *rate >= rand_num)
        .unwrap_or(&items[items.len() - 1]);

    let formatted_rate = format!("상한선: {}", rate);
    let formatted_item = style(item.to_owned()).with(color.to_owned()).to_string();
    let formatted_rand_num = style(format!("(난수: {})", rand_num)).with(Color::DarkGrey).to_string();

    format!("[{}] {}\n{}", formatted_rate, formatted_item, formatted_rand_num)
}
