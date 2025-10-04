use slugify::slugify;

fn main() {
    let text = "Café au lait! 2025";
    let slug = slugify(text);
    println!("Original: {}", text);
    println!("Slugified: {}", slug); // -> cafe_au_lait_2025
}
