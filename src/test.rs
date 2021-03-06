use parser;
use structures;

pub fn test_klassendiagramm(filename: String, debug: bool) {
    println!("Starte Klassendiagrammtest:\n");
    let success: bool;
    //println!("Enum: {:?}", parser::get_diagram_type(String::from("./KlassendiagrammForTesting.txt")));
    //let d = parser::parse_classes(String::from("./docs/Syntaxentwuerfe/KlassendiagrammForTesting.txt"), debug);
    let d = parser::parse_classes(filename, debug);
    let mut diagram = structures::Diagram {
        problems: Vec::new(),
        name: String::from("Fail"),
        packages: Vec::new(),
        nodes: Vec::new(),
        connections: Vec::new(),
    };
    match d {
        Ok(di) => {
            success = true;
            diagram = di;
        },
        Err(e) => {
            success = false;
            println!("Kritisches Problem: {:?}", e);
        },
    }
    if success {
        if debug {
            println!("\nFehlerliste:");
            for p in diagram.problems {
                println!("Problem: {:?}", p);
            }
        }
        println!("\nTitel: {}", diagram.name);
        println!("\nInhalt:");
        for pa in diagram.packages {
            println!("  Package: {}", pa.name);
            print_nodes("       ", pa.nodes);
            for conn in pa.connections {
                println!("      {} -> {}:", conn.node1, conn.node2);
                println!(
                    "          Art: {:?}, Kommentar: {}",
                    conn.contype, conn.description
                );
            }
            println!("");
        }
        println!("Klassen außerhalb von Packages:");
        print_nodes("   ", diagram.nodes);
        println!("Verbindungen außerhalb von Packages:");
        for conn in diagram.connections {
            println!("  {} -> {}:", conn.node1, conn.node2);
            println!(
                "      Art: {:?}, Kommentar: {}",
                conn.contype, conn.description
            );
        }
    }
}

fn print_nodes(ws: &str, nodes: Vec<structures::Node>) {
    for node in nodes {
        println!("{}{:?}: {} ", ws, node.nodetype, node.name);
        /*match (node.nodetype) {
            structures::Nodetype::CLASS => print!("Klasse: "),
            structures::Nodetype::ENUM => print!("Enumeration: "),
            structures::Nodetype::INTERFACE => print!("Interace: "),
            structures::Nodetype::ABSTRACT => print!("Abstrakte Klasse: "),
            structures::Nodetype::ANNOTATION => print!("Annotation: "),
            _ => ("Unbekannter Typ: "),
        }*/
        println!("Stereotyp: {}", node.stereotype);
        println!("Variablen:");
        for var in node.variables {
            println!("  {}{}", ws, var);
        }
        println!("Methoden:");
        for met in node.methods {
            println!("  {}{}", ws, met);
        }
        println!("Verbindungen:");
        println!("");
    }
}
