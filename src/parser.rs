use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use structures;

pub fn parse_classes(filename: String) -> structures::Problem {   
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    for (i, line) in file.lines().enumerate() {
        let l = line.unwrap();
        
    }
    return structures::Problem::NONE;
}

pub fn get_diagram_type(filename: String) -> structures::Diagramtype {
    let f = File::open(filename).unwrap();
    let mut buffer = BufReader::new(&f);
    let mut firstline = String::new();
    let suc = buffer.read_line(&mut firstline);
    //println!("{:?}", firstline);
    
    match firstline.as_ref() {
        "@startActivities\r\n" => return structures::Diagramtype::ACTIVITY,
        "@startUseCases\r\n" => return structures::Diagramtype::USECASE,
        "@startClasses\r\n" => return structures::Diagramtype::CLASSES,
        "@startSequence\r\n" => return structures::Diagramtype::SEQUENCE,
        _ => return structures::Diagramtype::NONE,
    }
}