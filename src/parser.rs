use regex::Regex;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use structures;
use regexCollection;
use regexCollection::Regexpiece;

pub fn parse_classes(filename: String, regexe: HashMap<regexCollection::Regexpiece, &str>,
problem: &structures::Problem) -> structures::Diagram {   
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);
    let mut diagram: structures::Diagram;
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        if num == 0 {
            let re = Regex::new(regexe.get(&Regexpiece::START)).unwrap();
            if !re.is_match(l) {
                *problem = structures::Problem::NOSTART;
                return diagram;
            }
        }
        
    }
    return diagram;
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

