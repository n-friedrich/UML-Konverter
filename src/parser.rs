extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
//use std::collections::HashMap;
use structures;
//use regexCollection;
//use regexCollection::Regexpiece;

pub fn parse_classes(filename: String, debug: bool) -> Result<structures::Diagram, structures::Problem> {
    let filename2 = filename.clone();
    if !File::open(filename.clone()).is_ok() {
        return Err(structures::Problem::NOFILE);
    }
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(&f);
    let file = reader.lines().enumerate();
    let mut got_start = false;

    let mut diagram = structures::Diagram {
        problems: Vec::new(),
        name: String::new(),
        packages: Vec::new(),
        nodes: Vec::new(),
        connections: Vec::new(),
    };

    let re_start = Regex::new(r"\s*@start\w").unwrap(); //geprüft
    let re_comment = Regex::new(r"^\s*#[^\n\r]*\s*(\r|\n)+$").unwrap(); //geprüft
    let re_whitespace = Regex::new(r"^\s*(\r|\n)+$").unwrap(); //geprüft
    let re_scopeentry = Regex::new(r"\s*\{\s*").unwrap(); //geprüft
    let re_title = Regex::new(r##"\s*title:"(\w+)""##).unwrap(); //geprüft
    let re_package = Regex::new(r##"\s*package:"(\w+)"\s*\{"##).unwrap();
    let re_node = Regex::new(r##"\s*(\w+)\s*:\s*(<[\w\s]*>)\s*"(\w+)"\s*(\{|,)"##).unwrap(); //geprüft
    let re_connections = Regex::new(r##"\s*connections\s*\{"##).unwrap(); //geprüft
    let re_docend = Regex::new(r"\s*@end").unwrap(); //geprüft

    let mut skip_to = 0;
    let mut finished = false;

    for (num, line) in file {
        let l = line.unwrap();
        if !got_start {
            if debug {println!("  Suche Start");}
            //Durchiterieren bis Start gefunden
            if re_start.is_match(&l) {
                got_start = true;
            } else {
                if re_docend.is_match(&l) {
                    return Err(structures::Problem::NOSTART);
                }
            }
        } else if re_title.is_match(&l) {
            //Titel gefunden
            if debug {println!("  Titel gefunden");}
            diagram.name = String::from(&re_title.captures(&l).unwrap()[1]);
        } else if !(num <= skip_to) && !finished {
            if debug {println!("Starte Zeile: {}:", num)}
            if re_docend.is_match(&l) {
                if debug {println!("  Ende gefunden");}
                finished = true;
            } else if re_package.is_match(&l) {
                //Package gefunden
                if debug {println!("  Package gefunden");}
                let pcache = read_package(
                    filename2.clone(),
                    num,
                    String::from(&re_package.captures(&l).unwrap()[1]),
                );
                diagram.packages.push(pcache.pack);
                for p in pcache.problems {
                    diagram.problems.push(p);
                }
                skip_to = pcache.fin_line;
            } else if re_node.is_match(&l) {
                //Node gefunden
                if debug {println!("  Node gefunden");}
                let ncache: Nodereturn;
                let ncapture = re_node.captures(&l);
                match ncapture {
                    Some(_) => {
                        ncache = read_node(filename2.clone(), num, re_node.captures(&l).unwrap());
                        diagram.nodes.push(ncache.new_node);
                        for p in ncache.problems {
                            diagram.problems.push(p);
                        }
                        skip_to = ncache.fin_line;
                    }
                    None => {
                        diagram.problems.push(structures::Problem::MISSINGARGUMENTS(num));
                        if re_scopeentry.is_match(&l) {
                            let search = search_scopeend(filename2.clone(), num);
                            if search.problem == structures::Problem::NONE {
                                skip_to = search.scopeend;
                            } else {
                                return Err(search.problem);
                            }
                        }
                    }
                }
            } else if re_connections.is_match(&l) {
                //Connections gefunden
                if debug {println!("  Connections gefunden");}
                let ccache = read_connections(filename2.clone(), num);
                for c in ccache.conns {
                    diagram.connections.push(c);
                }
                for p in ccache.problems {
                    diagram.problems.push(p);
                }
                skip_to = ccache.fin_line;
            } else if !re_comment.is_match(&l) && !re_whitespace.is_match(&l) {
                //Letzte Prüfung zum Entschließen ob Zeile falsch
                if debug {println!("  Nichts gefunden");}
                //diagram.problems.push(structures::Problem::WRONGLINE(num));
                if re_scopeentry.is_match(&l) {
                    let search = search_scopeend(filename2.clone(), num);
                    if search.problem == structures::Problem::NONE {
                        skip_to = search.scopeend;
                    } else {
                        return Err(search.problem);
                    }
                }
            }
        }
    }
    return Ok(diagram);
}

pub fn get_diagram_type(filename: String) -> Result<structures::Diagramtype, structures::Problem> {
    if File::open(filename.clone()).is_ok() {
        let f = File::open(filename).unwrap();
        let mut buffer = BufReader::new(&f);
        let mut firstline = String::new();
        let suc = buffer.read_line(&mut firstline);
        //println!("{:?}", firstline);

        if suc.is_ok() {
            match firstline.as_ref() {
                "@startActivities\r\n" => return Ok(structures::Diagramtype::ACTIVITY),
                "@startUseCases\r\n" => return Ok(structures::Diagramtype::USECASE),
                "@startClasses\r\n" => return Ok(structures::Diagramtype::CLASSES),
                "@startSequence\r\n" => return Ok(structures::Diagramtype::SEQUENCE),
                _ => return Err(structures::Problem::NOTYPE),
            }
        } else {
            return Err(structures::Problem::NOTYPE);
        }
    } else {
        return Err(structures::Problem::NOFILE);
    } 
}

fn read_package(filename: String, mut place: usize, pname: String) -> Packagereturn {
    let np = structures::Package {
        name: pname,
        nodes: Vec::new(),
        connections: Vec::new(),
    };
    let mut pr = Packagereturn {
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

fn read_node(filename: String, mut place: usize, ninhalt: regex::Captures) -> Nodereturn {
    let new_node = structures::Node {
        nodetype: structures::Nodetype::CLASS,
        name: String::from(&ninhalt[3]),
        stereotype: String::from(&ninhalt[2]),
        variables: Vec::new(),
        methods: Vec::new(),
    };
    let mut nr = Nodereturn {
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

fn read_classcontent(filename: String, place: usize) -> Classcontentreturn {
    let mut vr = Classcontentreturn {
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

fn read_connections(filename: String, place: usize) -> Connectionsreturn {
    let mut cr = Connectionsreturn {
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

fn search_scopeend(filename: String, mut place: usize) -> Scopeendreturn {
    let re_starts = Regex::new(r"\s*\{").unwrap(); //geprüft
    let re_ends = Regex::new(r"\s*\}").unwrap(); //geprüft
    let re_docend = Regex::new(r"\s*@end").unwrap(); //geprüft

    let mut sr = Scopeendreturn {
        scopeend: place.clone(),
        problem: structures::Problem::NONE,
    };
    let filename2 = filename.clone();
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(&f);

    for (num, line) in reader.lines().enumerate() {
        if !(num <= place) {
            let l = line.unwrap();
            if re_starts.is_match(&l) {
                let search = search_scopeend(filename2.clone(), num);
                if search.problem == structures::Problem::NONE {
                    place = search.scopeend;
                } else {
                    sr.problem = structures::Problem::NOENDOFSCOPE(sr.scopeend);
                    return sr;
                }
            } else if re_ends.is_match(&l) {
                sr.scopeend = num;
                return sr;
            } else if re_docend.is_match(&l) {
                sr.problem = structures::Problem::NOENDOFSCOPE(sr.scopeend);
                return sr;
            }
        }
    }

    return sr;
}

struct Packagereturn {
    pack: structures::Package,
    problems: Vec<structures::Problem>,
    fin_line: usize,
}
struct Nodereturn {
    new_node: structures::Node,
    problems: Vec<structures::Problem>,
    fin_line: usize,
}
struct Classcontentreturn {
    content: Vec<String>,
    fin_line: usize,
}
struct Connectionsreturn {
    conns: Vec<structures::Connection>,
    problems: Vec<structures::Problem>,
    fin_line: usize,
}

struct Scopeendreturn {
    scopeend: usize,
    problem: structures::Problem,
}
