extern crate rscb;

use rscb::word;

fn main() {
    let text = String::from("you say goodbye and i say hello .");
    println!("=====target text=====\n'{}'\n", text);
    word::words(text);
}
