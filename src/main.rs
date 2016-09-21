#[allow(dead_code)]
mod birl;
mod parser;

fn main() {
    println!("{} {} ─ {}", birl::NAME, birl::VERSION, birl::GREETING);
}
