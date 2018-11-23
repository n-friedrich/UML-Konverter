extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::iter::Enumerate;
//use std::collections::HashMap;
use structures;
//use regexCollection;
//use regexCollection::Regexpiece;

pub fn parse_classes(filename: String) -> structures::Diagram {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(&f);
    let file = reader.lines().enumerate();
    let mut has_title = false;
    let mut in_package = false;
    let mut in_node = false;
    let mut strCache: String;

    let mut diagram: structures::Diagram;
    diagram = structures::Diagram {
        problems: Vec::new(),
        name: String::new(),
        packages: Vec::new(),
        nodes: Vec::new(),
        connections: Vec::new(),
    };
    let mut package: structures::Package;
    package = structures::Package {
        name: String::new(),
        nodes: Vec::new(),
        connections: Vec::new(),
    };
    let mut node: structures::Node;
    node = structures::Node {
        name: String::new(),
        nodetype: structures::Nodetype::CLASS,
        stereotype: String::new(),
        variables: Vec::new(),
        methods: Vec::new(),
    };

    let re_comment = Regex::new(r"\s*(#[^\n\r]*\s*)?(\r|\n)+").unwrap();
    let re_title = Regex::new(r##"\s*title:"(\w+)""##).unwrap();
    let re_package = Regex::new(r##"\s*package:"(\w+)"\s*(\{|,)"##).unwrap();
    let re_class = Regex::new(r##"\s*class:(<\w+>)?"(\w+)"\s*(\{|,)"##).unwrap();

    let mut problemliste: Vec<structures::Problem> = Vec::new();

    for (num, line) in file {
        let l = line.unwrap();
        if num == 0 {
            let re = Regex::new(r"\s*@start\w").unwrap();
            if !re.is_match(&l) {
                problemliste.push(structures::Problem::NOSTART);
                diagram.problems = problemliste;
                return diagram;
            } else {
                //println!("Start gefunden!");
            }
        } else {
            if !has_title {
                if re_title.is_match(&l) {
                    diagram.name = String::from(&re_title.captures(&l).unwrap()[1]);
                    //println!("Titel gefunden!");
                    has_title = true;
                }
            }
            if in_package {
                //Neue Package Funktion unten
                if in_node {

                } else {
                    if re_class.is_match(&l) {
                        //Klasse innerhalb von Packages
                        let cache = re_class.captures(&l).unwrap();
                        node = structures::Node {
                            name: String::from(&cache[2]),
                            nodetype: structures::Nodetype::CLASS,
                            stereotype: String::from(&cache[1]),
                            variables: Vec::new(),
                            methods: Vec::new(),
                        };
                        if cache[2] == *"," {
                            package.nodes.push(node);
                        } else {
                            in_node = true;
                        }
                    }
                }
            } else if in_node {

            } else {
                if re_package.is_match(&l) {
                    //Package
                    let cache = re_package.captures(&l).unwrap();
                    package = structures::Package {
                        name: String::from(&cache[1]),
                        nodes: Vec::new(),
                        connections: Vec::new(),
                    };
                    if cache[2] == *"," {
                        sendpkg = structures::Package {
                            name: String::from(package.name),
                            nodes: package.nodes.Clone(),
                        };
                        diagram.packages.push(sendpkg);
                    } else {
                        in_package = true;
                    }
                } else if re_class.is_match(&l) {
                    //Klasse außerhalb von Packages
                    let cache = re_class.captures(&l).unwrap();
                    node = structures::Node {
                        name: String::from(&cache[2]),
                        nodetype: structures::Nodetype::CLASS,
                        stereotype: String::from(&cache[1]),
                        variables: Vec::new(),
                        methods: Vec::new(),
                    };
                    if cache[2] == *"," {
                        diagram.nodes.push(node);
                    } else {
                        in_node = true;
                    }
                }
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

fn read_package(filename: String, place: usize, pname: String) -> package_return {
    let mut np = structures::Package {
        name: pname,
        nodes: Vec::new(),
        connections: Vec::new(),
    };
    let mut pr = package_return {
        pack: np,
        fin_line: place,
    };

    let re_node = Regex::new(r##"\s*(\w+)\s*:\s*(<[\w\s]+>)?\s*"(\w+)"\s*(\{|,)"##).unwrap();
    let re_comment = Regex::new(r"\s*(#[^\n\r]*\s*)?(\r|\n)+").unwrap();
    let re_end = Regex::new(r##"\s*\}"##).unwrap();

    let f = File::open(filename).unwrap();
    let reader = BufReader::new(&f);
    let mut finished = false;
    for (num, line) in reader.lines().enumerate() {
        if !(num <= place) && !finished {
            let l = line.unwrap();
            if re_node.is_match(&l) {
                let cache = re_node.captures(&l).unwrap();
            }
        }
    }

    return pr;
}

fn read_node(filename: String, place: usize, ninhalt: regex::Captures) -> node_return {
    let mut new_node = structures::Node {
        nodetype: structures::Nodetype::CLASS,
        name: String::from(&ninhalt[3]),
        stereotype: String::from(&ninhalt[2]),
        variables: Vec::new(),
        methods: Vec::new(),
        connections: Vec::new(),
    };
    let mut nr = node_return {
        new_node: new_node,
        problems: Vec::new(),
        fin_line: place,
    };

    match &ninhalt[1] {
        "class" => nr.new_node.nodetype = structures::Nodetype::CLASS,
        "enum" => nr.new_node.nodetype = structures::Nodetype::ENUM,
        "interface" => nr.new_node.nodetype = structures::Nodetype::INTERFACE,
        "abstract" => nr.new_node.nodetype = structures::Nodetype::ABSTRACT,
        "annotation" => nr.new_node.nodetype = structures::Nodetype::ANNOTATION,
        _ => nr.new_node.nodetype = structures::Nodetype::UNKNOWN(place),
    }
    if ninhalt[4] == *"," {
        nr.fin_line = place;
    } else {
        let mut finished = false;
        let re_varmet = Regex::new(r##"\s*(\w+)\s*\{"##).unwrap(); //geprüft
        let re_end = Regex::new(r##"\s*\}"##).unwrap(); //geprüft

        let f = File::open(filename).unwrap();
        let reader = BufReader::new(&f);
        for (num, line) in reader.lines().enumerate() {
            if !(num <= place) && !finished {
                let l = line.unwrap();
                if re_end.is_match(&l) {
                    finished = true;
                    nr.fin_line = num;
                } else if re_varmet.is_match(&l) {
                    let cache = re_varmet.captures(&l).unwrap();
                    match &cache[1] {
                        "variables" => {
                            let ca_var = read_classcontent(filename, num);
                            nr.new_node.variables = ca_var.content;
                            place = ca_var.fin_line;
                        }
                        "methods" => {
                            let ca_met = read_classcontent(filename, num);
                            nr.new_node.methods = ca_met.content;
                            place = ca_met.fin_line;
                        }
                        "connections" => {
                            let ca_conn = read_connections(filename, num);
                            nr.new_node.connections = ca_conn.conns;
                            place = ca_conn.fin_line;
                            for p in ca_conn.problems {nr.problems.push(p);}
                        }
                        _ => nr.problems.push(structures::Problem::UNKNOWN(num)),
                    }
                }
            }
        }
    }

    return nr;
}

fn read_classcontent(filename: String, place: usize) -> classcontent_return {
    let mut vr = classcontent_return {
        content: Vec::new(),
        fin_line: place,
    };

    let mut finished = false;
    let re_newvar = Regex::new(r##"\s*"([^=/".,;^°]+)"\s*,"##).unwrap();
    let re_end = Regex::new(r##"\s*\}"##).unwrap();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(&f);
    for (num, line) in reader.lines().enumerate() {
        if !(num <= place) && !finished {
            let l = line.unwrap();
            if re_end.is_match(&l) {
                finished = true;
                vr.fin_line = num;
            } else if re_newvar.is_match(&l) {
                vr.content
                    .push(String::from(&re_newvar.captures(&l).unwrap()[1]));
            }
        }
    }

    return vr;
}

fn read_connections(filename: String, place: usize) -> connections_return {
    let mut cr = connections_return {
        conns: Vec::new(),
        problems: Vec::new(),
        fin_line: place,
    };

    let mut finished = false;
    let re_conn = Regex::new(r##"\s*"(\w+)"\s*([|<>-]+)\s*('[^']*')?\s*"(\w+)"\s*,"##).unwrap(); //geprüft
    let re_end = Regex::new(r##"\s*\}"##).unwrap();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(&f);

    for (num, line) in reader.lines().enumerate() {
        if !(num <= place) && !finished {
            let l = line.unwrap();
            if re_end.is_match(&l) {
                finished = true;
                cr.fin_line = num;
            } else if re_conn.is_match(&l) {
                let cache = re_conn.captures(&l).unwrap();
                let conntype: structures::Conntype;
                match &cache[2] {
                    "|>" => conntype = structures::Conntype::VERERBUNG,
                    "->" => conntype = structures::Conntype::INTERFACE,
                    "-" => conntype = structures::Conntype::GESTRICHELT,
                    ">" => conntype = structures::Conntype::BEINHALTET,
                    "<>" => conntype = structures::Conntype::AGGREGATION,
                    "<|>" => conntype = structures::Conntype::KOMPOSITION,
                    _ => {
                        conntype = structures::Conntype::GESTRICHELT;
                        cr.problems.push(structures::Problem::UNKNOWN(num));
                    }
                }
                cr.conns.push(structures::Connection {
                    node1: String::from(&cache[1]),
                    node2: String::from(&cache[4]),
                    contype: conntype,
                    description: String::from(&cache[3]),
                });
            }
        }
    }

    return cr;
}

struct package_return {
    pack: structures::Package,
    fin_line: usize,
}
struct node_return {
    new_node: structures::Node,
    problems: Vec<structures::Problem>,
    fin_line: usize,
}
struct classcontent_return {
    content: Vec<String>,
    fin_line: usize,
}
struct connections_return {
    conns: Vec<structures::Connection>,
    problems: Vec<structures::Problem>,
    fin_line: usize,
}
