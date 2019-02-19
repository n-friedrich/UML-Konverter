extern crate regex;
extern crate image;
extern crate imageproc;
extern crate rusttype;

use image::{Rgb,RgbImage};
use imageproc::definitions::Image;
use class_uml::draw_diagramm;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use std::env;

mod class_uml;
mod structures;
mod parser;
mod test;
//mod regexCollection;

fn main() {
    let args: Vec<String> = env::args().collect();
    test::test_klassendiagramm(args[1].clone(), true);
  //das Bild
  

    

    println!("Starte Klassendiagrammtest:\n");
    let success: bool;
    let d = parser::parse_classes(args[1].clone(), false);
    //let d = parser::parse_classes("docs/Syntaxentwuerfe/KlassendiagrammForTesting.txt".to_string(), false);
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
        for p in &diagram.problems {
            println!("Problem: {:?}", p);
        }
        println!("Das Diagramm {} wurde erfolgreich geparsed. Die Bilderstellung wird gestartet...", diagram.name);
          
         let white = Rgb([255u8, 255u8, 255u8]);
         let mut image = RgbImage::new(1600, 1000);
         let whiteboard = Rect::at(0,0).of_size(1600,1000);
         let create_d = &mut diagram.clone();
         draw_filled_rect_mut(&mut image,whiteboard,white);
        draw_diagramm(create_d, &mut image);
         image.save("uml.png").unwrap();
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

