extern crate image;
extern crate imageproc;
extern crate structures;

use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::drawing::draw_text_mut;
use imageproc::rect::Rect;
use image::{ImageBuffer,Rgb,RgbImage};


fn draw_package(){
//to do

}

pub struct Node {
    //Fuer Klassen und Anwendungen
    typus: String, //Nodetyp (Name) evtl erstetzen mit enum
    name: String, //Name des Nodes
    stereotype: String, //Stereotyp des Nodes
    variables: Vec<String>, //Liste mit Variablen
    methods: Vec<String>, //Liste mit Methoden
}

//malen eines einzelnen Klassenobjekts
//classuml enthält den Inhalt der Klasse
fn draw_classuml(classuml:Node, uml:RpgImage,pos1:int,pos2:int){
    let textsize = Scale::new(20);
    let stdfont = Font::new();
    let rect = Rect::at(pos1,pos2).of_size(200,200);
    draw_text_mut(&mut img,Rgb,120,120 ,textsize, &stdfont,"Inhalt");
    draw_hollow_rect_mut(&mut img, rect,Rgb([255, 255, 255]));

}

fn main() {
    let mut img = RgbImage::new(1000,1000);
//test node
    let testlist= Vec::<String>= ["1","2","3","4","5"];
    let methodlist = Vec::<String> = ["m1","m2","m3"];
 /*   let test = Node {
        typus:String::from("help"),
        name: String::from("Testklasse"),
        stereotype: String::from("Hope"),
        variables: testlist<>;
        methods: methodlist<>;
    }; */
    draw_classuml(test,img,100,100);
    img.save("UML.png").unwrap();
}
