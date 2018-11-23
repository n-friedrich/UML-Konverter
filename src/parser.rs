extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::collections::HashMap;
use structures;
//use regexCollection;
//use regexCollection::Regexpiece;

pub fn parse_classes(filename: String) -> structures::Diagram {
    let filename2 = filename.clone();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(&f);
    let file = reader.lines().enumerate();
    let mut has_title = false;

    let mut diagram = structures::Diagram {
        problems: Vec::new(),
        name: String::new(),
        packages: Vec::new(),
        nodes: Vec::new(),
        connections: Vec::new(),
    };

    //let re_comment = Regex::new(r"\s*(#[^\n\r]*\s*)?(\r|\n)+").unwrap();
    let re_title = Regex::new(r##"\s*title:"(\w+)""##).unwrap(); //geprüft
    let re_package = Regex::new(r##"\s*package:"(\w+)"\s*\{"##).unwrap();
    let re_node = Regex::new(r##"\s*(\w+)\s*:\s*(<[\w\s]*>)\s*"(\w+)"\s*(\{|,)"##).unwrap(); //geprüft
    let re_connections = Regex::new(r##"\s*connections\s*\{"##).unwrap(); //geprüft
    let re_end = Regex::new(r"\s*@end").unwrap(); //geprüft

    let mut skip_to = 0;
    let mut finished = false;

    for (num, line) in file {
        let l = line.unwrap();
        if num == 0 {
            let re = Regex::new(r"\s*@start\w").unwrap(); //geprüft
            if !re.is_match(&l) {
                diagram.problems.push(structures::Problem::NOSTART);
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
            } else if !(num <= skip_to) && !finished {
                if re_end.is_match(&l) {
                    finished = true;
                } else if re_package.is_match(&l) {
                    let pcache = read_package(filename2.clone(), num, String::from(&re_package.captures(&l).unwrap()[1]));
                    diagram.packages.push(pcache.pack);
                    for p in pcache.problems {
                        diagram.problems.push(p);
                    }
                    skip_to = pcache.fin_line;
                } else if re_node.is_match(&l) {
                    let ncache = read_node(filename2.clone(), num, re_node.captures(&l).unwrap());
                    diagram.nodes.push(ncache.new_node);
                    for p in ncache.problems {
                        diagram.problems.push(p);
                    }
                    skip_to = ncache.fin_line;
                } else if re_connections.is_match(&l) {
                    let ccache = read_connections(filename2.clone(), num);
                    for c in ccache.conns {
                        diagram.connections.push(c);
                    }
                    for p in ccache.problems {
                        diagram.problems.push(p);
                    }
                    skip_to = ccache.fin_line;
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

fn read_package(filename: String, mut place: usize, pname: String) -> package_return {
    let np = structures::Package {
        name: pname,
        nodes: Vec::new(),
        connections: Vec::new(),
    };
    let mut pr = package_return {
        pack: np,
        problems: Vec::new(),
        fin_line: place,
    };

    let re_node = Regex::new(r##"\s*(\w+)\s*:\s*(<[\w\s]*>)\s*"(\w+)"\s*(\{|,)"##).unwrap(); //geprüft
    let re_connections = Regex::new(r##"\s*connections\s*\{"##).unwrap(); //geprüft
    //let re_comment = Regex::new(r"\s*(#[^\n\r]*\s*)?(\r|\n)+").unwrap();
    let re_end = Regex::new(r##"\s*\}"##).unwrap(); //geprüft

    let filename2 = filename.clone();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(&f);
    let mut finished = false;
    for (num, line) in reader.lines().enumerate() {
        if !(num <= place) && !finished {
            let l = line.unwrap();
            if re_end.is_match(&l) {
                finished = true;
                pr.fin_line = num;
            } else if re_node.is_match(&l) {
                let node_cache = read_node(filename2.clone(), num, re_node.captures(&l).unwrap());
                for p in node_cache.problems {
                    pr.problems.push(p);
                }
                place = node_cache.fin_line;
                pr.pack.nodes.push(node_cache.new_node);
            } else if re_connections.is_match(&l) {
                let con_cache = read_connections(filename2.clone(), num);
                for p in con_cache.problems {
                    pr.problems.push(p);
                }
                place = con_cache.fin_line;
                for conn in con_cache.conns {
                    pr.pack.connections.push(conn);
                }
            }
        }
    }

    return pr;
}

fn read_node(filename: String, mut place: usize, ninhalt: regex::Captures) -> node_return {
    let new_node = structures::Node {
        nodetype: structures::Nodetype::CLASS,
        name: String::from(&ninhalt[3]),
        stereotype: String::from(&ninhalt[2]),
        variables: Vec::new(),
        methods: Vec::new(),
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

        let filename2 = filename.clone();
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
                            let ca_var = read_classcontent(filename2.clone(), num);
                            for var in ca_var.content {
                                nr.new_node.variables.push(var);
                            }
                            place = ca_var.fin_line;
                        }
                        "methods" => {
                            let ca_met = read_classcontent(filename2.clone(), num);
                            for met in ca_met.content {
                                nr.new_node.methods.push(met);
                            }
                            place = ca_met.fin_line;
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
    let re_conn = Regex::new(r##"\s*"(\w+)"\s*([|<>-]+)\s*('[^']*')\s*"(\w+)"\s*,"##).unwrap(); //geprüft
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
    problems: Vec<structures::Problem>,
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
