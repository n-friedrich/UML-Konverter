
extern crate image;
extern crate imageproc;
extern crate rusttype;

mod structures;
mod class_uml;



use std::path::Path;
//use imageproc;
//use imageproc::drawing::draw_hollow_rect_mut;
//use imageproc::drawing::draw_text_mut;
use imageproc::rect::Rect;
use image::{ImageBuffer,Rgb,RgbImage,GenericImage};
use imageproc::definitions::Image;
use imageproc::drawing::draw_cross_mut;
use imageproc::drawing::draw_filled_rect_mut;
use structures::Nodetype;
use class_uml::draw_classuml;



use std::env;
use imageproc::drawing::draw_text_mut;




fn main() {

    let red   = Rgb([255u8, 0u8,   0u8]);
    let green = Rgb([0u8,   255u8, 0u8]);
    let blue  = Rgb([0u8,   0u8,   255u8]);
    let white = Rgb([255u8, 255u8, 255u8]);

   // draw_cross_mut(&mut img, white, 5, 5);


    //let mut img = GenericImage::new(1600,1000);
    let vartest = vec!["Test Variable".to_string(), "Test Variable 2".to_string()];
    let mettest = vec!["Test Methode".to_string(), "Test Methode".to_string()];
//test node

    let test = structures::Node {
        nodetype: Nodetype::CLASS,
        name: String::from("Testklasse"),
        stereotype: String::from("Hope"),
        variables: vartest,
        methods: mettest,
    };
   // draw_classuml(test, 100, 100);






    let mut image = RgbImage::new(1600, 1000);
    let whiteboard = Rect::at(0,0).of_size(1600,1000);
    draw_filled_rect_mut(&mut image,whiteboard,white);
  //  let font = Vec::from(include_bytes!("Alef-Regular.ttf") as &[u8]);
 //   let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
    draw_classuml(test,10,10,&mut image);
 /*
    let height = 12.4;
    let scale = Scale { x: height * 2.0, y: height };
    draw_text_mut(&mut image, Rgb([0u8, 0u8, 255u8]), 0, 0, scale, &font, "Hello, world!");
 */
    image.save("uml.png").unwrap();

}