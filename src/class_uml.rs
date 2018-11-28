/*
extern crate image;
extern crate imageproc;
extern crate rusttype;
*/
use imageproc::definitions::Clamp;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::drawing::draw_text_mut;
use imageproc::drawing;
use imageproc::rect::Rect;
use imageproc::definitions::Image;
use rusttype::{FontCollection, Scale};
use image::{ImageBuffer,Rgb,RgbImage,GenericImage,GenericImageView};
use structures;
use std::iter::Iterator;
use image::Pixel;




fn draw_package(umlpack:structures::Package,  img:&mut RgbImage){

    //die Pixel Level der Ebenen im moment Feste Werte
    let mut level1 = 100;
    let mut level2 = 300;
    let mut level3 = 500;
    let mut level4 = 700;
    //eine Liste an nodes für die Jeweiligen Ebenen 
    let mut level1node:Vec<structures::Node>;
    let mut level2node:Vec<structures::Node>;
    let mut level3node:Vec<structures::Node>;
    let mut level4node:Vec<structures::Node>;

    let mut amount:i32;
    let mut form =amount/4;

    //listen einteilen mithilfe der connections
    let mut set = 0;

    for node in umlpack.nodes{
        set = 0;
        let mut not1 = 0;
        let mut not2 = 0;
        let mut not3 = 0;
        let mut not4 = 0;
        for conn in umlpack.connections{

            if(node.name ==conn.node2 && set == 0){
                not4 = 1;
                set = 0;
            }

        }

        if(level1node.len()<4 && not1 == 0){
            level1node.push(node)
        }else if(level2node.len()<4 && not2 == 0) {
            level2node.push(node)
        }else if(level3node.len()<4 && not3 == 0) {
            level3node.push(node)
        }else if(level4node.len()<4 && not4 == 0) {
            level4node.push(node)
        }


    }


    let mut pos_W = 0;
    let mut pos_H = 0;
    for node in level1node{
         draw_classuml(Node,,img);
    }

    for node in level2node{
        draw_classuml(Node,,img);
    }

    for node in level3node{
        draw_classuml(Node,,img);
    }

    for node in level4node{
        draw_classuml(Node,,img);
    }
}




    /* if(connection.node2==nodename ){
    drawuml

    */






//malen eines einzelnen Klassenobjekts
//classuml enthält den Inhalt der Klasse
pub fn draw_classuml(class_node:structures::Node, pos1: u32, pos2: u32, img:&mut RgbImage){

    let red   = Rgb([255u8, 0u8,   0u8]);
    let green = Rgb([0u8,   255u8, 0u8]);
    let blue  = Rgb([0u8,   0u8,   255u8]);
    let white = Rgb([255u8, 255u8, 255u8]);
    let black = Rgb([0u8, 0u8, 0u8]);

    let mut methodline:u32 = 0;
    let mut titleline: u32 = 20;

    let mut rectsize_w:u32 = 100;
    let mut rectsize_h:u32 = 100;


    let font = Vec::from(include_bytes!("Alef-Regular.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();


    let newline = 15;
    let mut textline:u32 = titleline+newline;
    let mut textwidth:u32 = pos1+5;


    let stdfont=&font;
    let textsize = Scale{x:16 as f32,y:14 as f32};
    let titlesize = Scale{x:17 as f32,y:19 as f32};

    //Titel malen
    draw_text_mut(img,black,textwidth+newline,(pos2 + 2 as u32) ,titlesize, stdfont,  &class_node.name);


    //Variablenliste
    for line in class_node.variables {
        draw_text_mut(img,black,textwidth,textline ,textsize, stdfont,  &*line);
        textline += newline;
    }

    methodline = textline+(newline/2 as u32)-newline ;
    textline = methodline+newline;
    //methodenliste
    for line in class_node.methods {
        draw_text_mut(img,black,textwidth,textline ,textsize, stdfont,  &*line);
        textline += newline;

    }

    let rect = Rect::at(pos1 as i32,pos2 as i32).of_size(rectsize_w ,rectsize_h);
    drawing::draw_line_segment_mut( img,(pos1 as f32,pos2 as f32+titleline as f32),(pos1 as f32 +(rectsize_w-1) as f32,pos2 as f32 + titleline as f32),black);
    drawing::draw_line_segment_mut( img,(pos1 as f32,pos2 as f32+methodline as f32),(pos1 as f32 +(rectsize_w-1) as f32,pos2 as f32+methodline as f32),black);
    draw_hollow_rect_mut(   img , rect,black);
}


