extern crate regex;
use std::env;
mod structures;
mod parser;
mod test;
//mod regexCollection;

fn main() {
    let args: Vec<String> = env::args().collect();
    //test::test_klassendiagramm(args[1].clone(), true);

    println!("Starte Klassendiagrammtest:\n");
    let success: bool;
    let d = parser::parse_classes(args[1].clone(), false);
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
            println!("Kritischer Fehler, der zum vorzeitigen Beenden des Programms geführt hat:");
            match e {
                structures::Problem::NOFILE => println!("Die zu bearbeitende Datei konnte nicht gefunden oder geöffnet werden!"),
                structures::Problem::NOTYPE => println!("In der zu bearbeitenden Datei konnte kein Diagrammtyp gefunden werden!"),
                structures::Problem::NOSTART => println!("In der zu bearbeitenden Datei konnte kein Startpunkt gefunden werden!"),
                structures::Problem::NOENDOFSCOPE(line) => println!("In der zu bearbeitenden Datei konnte kein Ende für das Scope,
                 das in Zeile {} geöffnet wird, gefunden werden!", line),
                _ => println!("{:?}", e),
            }
        },
    }
    if success {
        println!("\nFehlerliste:");
        for p in diagram.problems {
            println!("Problem: {:?}", p);
        }
        println!("Das Diagramm {} wurde erfolgreich geparsed. Die Bilderstellung wird gestartet...", diagram.name);

        /* 
         * ----------------------------------------------------------------
         * -#-#-#-#-#- Hier das Diagramm zum Bild parsen lassen -#-#-#-#-#-
         * ----------------------------------------------------------------
         */

        //Bei Erfolg:
        println!("\nDas Bild wurde erfolreich erstellt.");

        /* 
         * ---------------------------------------------------------------------------------------------
         * -#-#-#-#-#- Hier das Bild öffnen oder, falls nicht möglich, Verweis darauf zeigen -#-#-#-#-#-
         * ---------------------------------------------------------------------------------------------
         */
        
    }
}
