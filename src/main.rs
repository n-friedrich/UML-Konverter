//extern crate regex;

mod structures;
mod parser;
mod regexCollection;

fn main() {
    //println!("Enum: {:?}", parser::get_diagram_type(String::from("./KlassendiagrammForTesting.txt")));
    let regexe = regexCollection::regex_collection_init();
}
