extern crate regex;

mod structures;
mod parser;
//mod regexCollection;

fn main() {
    //println!("Enum: {:?}", parser::get_diagram_type(String::from("./KlassendiagrammForTesting.txt")));
    //let regexe = regexCollection::regex_collection_init();
    let diagram: structures::Diagram = parser::parse_classes(String::from("./KlassendiagrammForTesting.txt"));
    println!("Fehlerliste:");
    for p in diagram.problems {
        println!("Problem: {:?}", p);
    }
    println!("Titel: {}", diagram.name);
}
