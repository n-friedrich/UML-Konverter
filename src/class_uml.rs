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

//Farben werden häufig genutzt
/*
static RED:Rgb = [255u8, 0u8,   0u8];
static GREEN:Rgb = Rgb([0u8,   255u8, 0u8]);
static BLUE:Rgb  = Rgb([0u8,   0u8,   255u8]);
static WHITE:Rgb = Rgb([255u8, 255u8, 255u8]);
static BLACK:Rgb = Rgb([0u8, 0u8, 0u8]);
*/
pub fn draw_diagramm(dia: &mut structures::Diagram, img:&mut RgbImage){

    let textsize = Scale{x:16 as f32,y:14 as f32};
    let black = Rgb([0u8, 0u8, 0u8]);

    let font = Vec::from(include_bytes!("Alef-Regular.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();

    let mut probtext = 50;
    draw_text_mut(img,black,probtext,950 ,textsize, &font,  "Problem:");
    probtext += 100;

    for prob in &mut dia.problems{

        //draw_text_mut(img,black,probtext,950 ,textsize, &font,  &*prob);
        probtext+110;
    }



    for pack in &mut dia.packages{
        draw_package(pack,img);
    }

}


fn draw_package( umlpack: &mut structures::Package,  img:&mut RgbImage){

    let black = Rgb([0u8, 0u8, 0u8]);

    let mut connecti:Vec<structures::Connpoints> = vec![];

    let font = Vec::from(include_bytes!("Alef-Regular.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();



    //die Pixel Level der Ebenen im moment Feste Werte
    let mut level1 = 100;
    let mut level2 = 300;
    let mut level3 = 500;
    let mut level4 = 700;
    //eine Liste an nodes für die Jeweiligen Ebenen 
    let mut level1node:Vec<structures::Node> = vec![];
    let mut level2node:Vec<structures::Node> = vec![];
    let mut level3node:Vec<structures::Node> = vec![];
    let mut level4node:Vec<structures::Node> = vec![];

    let mut level1point:Vec<structures::Point>  = vec![];
    let mut level2point:Vec<structures::Point>  = vec![];
    let mut level3point:Vec<structures::Point>  = vec![];

    let mut level2endpoint:Vec<structures::Point>  = vec![];
    let mut level3endpoint:Vec<structures::Point>  = vec![];
    let mut level4endpoint:Vec<structures::Point> = vec![];

    let mut amount:i32;
    amount = umlpack.nodes.len() as i32;
    let mut form =amount/4;

    //listen einteilen mithilfe der connections
    let mut set = 0;

    for node in &umlpack.nodes{
        set = 0;
        let mut not1 = 0;
        let mut not2 = 0;
        let mut not3 = 0;
        let mut not4 = 0;
        for conn in &umlpack.connections{

            if(node.name ==conn.node1){
                not4 = 1;

            }
            if(node.name ==conn.node2){
                not1 = 1;

            }

        }

        if(level1node.len()<4 && not1 == 0){
            level1node.push(node.to_owned())
        }else if(level2node.len()<4 && not2 == 0) {
            level2node.push(node.to_owned())
        }else if(level3node.len()<4 && not3 == 0) {
            level3node.push(node.to_owned())
        }else if(level4node.len()<4 && not4 == 0) {
            level4node.push(node.to_owned())
        }


    }

    //position für die connections

    //zeichnen der Klassen
    let mut pos_w = 50;
    let mut w_step = 300;
    //let mut i:u32 = 0;
    for node in &level1node{
        let nodee = node.to_owned();
         level1point.push( draw_classuml(nodee,pos_w,level1,img) );
        pos_w += w_step+50;

    }
    pos_w = 50;

    for node in &level2node{
        let nodee = node;
        level2point.push( draw_classuml(node.clone(),pos_w,level2,img) );
        level2endpoint.push(structures::Point{x:pos_w,y:level2} );
        pos_w += w_step+50;
    }
    pos_w = 50;

    for node in &level3node{
        let nodee = node;
        level3point.push( draw_classuml(node.clone(),pos_w,level3,img) );
        level3endpoint.push(structures::Point{x:pos_w,y:level3});
        pos_w += w_step+50;
    }
    pos_w = 50;

    for node in &level4node{
        let nodee = node;
        draw_classuml(node.clone(),pos_w,level4,img);
        level4endpoint.push(structures::Point{x:pos_w,y:level4});
        pos_w += w_step+50;
    }


    //umlpack.connections.push(connpoints::structures(lev));
/*
    for conn in &mut umlpack.connections{
        let mut i:usize = 0;
        let mut j:usize = 0;
        for node_s_temp in &mut level1node  {
            let node_s =node_s_temp;
            if(conn.node1==node_s.name){

                for node_e in &mut level2node{
                    if(conn.node2==node_e.name){
                        let temp1 = level1point[i].clone();
                        let temp2 = &level2endpoint[j];
                    connecti.push(structures::Connpoints{
                        start_x:temp1.x,
                        start_y:temp1.y,
                        end_x:temp2.x,
                        end_y:temp2.y,
                        connection:conn.clone(),
                    })
                    }
                }

                j = 0;

                for node_e in level3node{
                    if(conn.node2==node_e.name){
                        let temp1 = &level1point[i];
                        let temp2 = &level2endpoint[j];
                        connecti.push(structures::Connpoints{
                            start_x:temp1.x,
                            start_y:temp1.y,
                            end_x:temp2.x,
                            end_y:temp2.y,
                            connection:conn.clone(),
                        })
                    }
                }

                j = 0;
                for node_e in level4node{
                    if(conn.node2==node_e.name){
                        let temp1 = &level1point[i];
                        let temp2 = &level2endpoint[j];
                        connecti.push(structures::Connpoints{
                            start_x:temp1.x,
                            start_y:temp1.y,
                            end_x:temp2.x,
                            end_y:temp2.y,
                            connection:conn.clone(),
                        })
                    }
                }

                j = 0;
            }
            i+1;
        }

        i = 0;
////////level 2 nodes
        for node_s in level2node  {
            if(conn.node1==node_s.name){

                for node_e in level3node{
                    if(conn.node2==node_e.name){
                        let temp1 = &level2point[i];
                        let temp2 = &level3endpoint[j];
                        connecti.push(structures::Connpoints{
                            start_x:temp1.x,
                            start_y:temp1.y,
                            end_x:temp2.x,
                            end_y:temp2.y,
                            connection:conn.clone(),
                        })
                    }
                }

                j = 0;
                for node_e in level4node{
                    if(conn.node2==node_e.name){
                        let temp1 = &level2point[i];
                        let temp2 = &level3endpoint[j];
                        connecti.push(structures::Connpoints{
                            start_x:temp1.x,
                            start_y:temp1.y,
                            end_x:temp2.x,
                            end_y:temp2.y,
                            connection:conn.clone(),
                        })
                    }
                }

                j = 0;
            }
            i+1;
        }
        i = 0;
/// level 3 nodes
        for node_s in level3node  {
            if(conn.node1==node_s.name){

                j = 0;
                for node_e in level4node{
                    if(conn.node2==node_e.name){
                        let temp1 = &level3point[i];
                        let temp2 = &level4endpoint[j];
                        connecti.push(structures::Connpoints{
                            start_x:temp1.x,
                            start_y:temp1.y,
                            end_x:temp2.x,
                            end_y:temp2.y,
                            connection:conn.clone(),
                        })
                    }
                }

                j = 0;
            }
            i+1;
        }
        i = 0;
    }

    for conn in connecti{
        draw_connection(conn,img);
    }
*/
}



pub fn draw_connection(connpoints:structures::Connpoints,img:&mut RgbImage,){

    let black = Rgb([0u8, 0u8, 0u8]);

    drawing::draw_line_segment_mut(img,(connpoints.start_x as f32,connpoints.start_y as f32),(connpoints.start_x as f32,(connpoints.start_y+10) as f32),black);


    if (connpoints.connection.contype == structures::Conntype::VERERBUNG){
        drawing::draw_line_segment_mut(img,(connpoints.start_x as f32,connpoints.start_y as f32),(connpoints.start_x as f32,(connpoints.start_y+10) as f32),black);
    }else if (connpoints.connection.contype == structures::Conntype::INTERFACE) {
        drawing::draw_line_segment_mut(img,(connpoints.start_x as f32,connpoints.start_y as f32),(connpoints.start_x as f32,(connpoints.start_y+10) as f32),black);
    }else if (connpoints.connection.contype == structures::Conntype::GESTRICHELT) {
        drawing::draw_line_segment_mut(img,(connpoints.start_x as f32,connpoints.start_y as f32),(connpoints.start_x as f32,(connpoints.start_y+10) as f32),black);
    }else if (connpoints.connection.contype == structures::Conntype::BEINHALTET) {
        drawing::draw_line_segment_mut(img,(connpoints.start_x as f32,connpoints.start_y as f32),(connpoints.start_x as f32,(connpoints.start_y+10) as f32),black);
    }else if (connpoints.connection.contype == structures::Conntype::KOMPOSITION) {
        drawing::draw_line_segment_mut(img,(connpoints.start_x as f32,connpoints.start_y as f32),(connpoints.start_x as f32,(connpoints.start_y+10) as f32),black);
    }else if (connpoints.connection.contype == structures::Conntype::AGGREGATION) {

    }

}


//malen eines einzelnen Klassenobjekts
//classuml enthält den Inhalt der Klasse
pub fn draw_classuml(class_node:structures::Node, pos1: u32, pos2: u32, img:&mut RgbImage)->structures::Point{

    let black = Rgb([0u8, 0u8, 0u8]);
    let newline = 15;

    let mut titleline: u32 = pos2;

    let mut rectsize_w:u32 = 250;
    let mut rectsize_h:u32 = 150;


    let font = Vec::from(include_bytes!("Alef-Regular.ttf") as &[u8]);
    let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();


    let mut textline:u32 = 0;
    let mut textwidth:u32 = 0;
    textline = titleline+25;
    textwidth = pos1+5;


    let stdfont=&font;
    let textsize = Scale{x:16 as f32,y:14 as f32};
    let titlesize = Scale{x:17 as f32,y:19 as f32};

    //Titel malen
    draw_text_mut(img,black,textwidth+20,(titleline as u32) ,titlesize, stdfont,  &class_node.name);

    drawing::draw_line_segment_mut( img,(pos1 as f32,(titleline+20) as f32),(pos1 as f32 +(rectsize_w-1) as f32,(titleline+20) as f32),black);


    //Variablenliste
    for line in class_node.variables {
        draw_text_mut(img,black,textwidth,textline ,textsize, stdfont,  &*line);
        textline += newline;
    }


    if(class_node.methods.len()>0){
        drawing::draw_line_segment_mut( img,(pos1 as f32,textline as f32),(pos1 as f32 +(rectsize_w-1) as f32,textline as f32),black);
        textline += newline;
    }


    //methodenliste
    for line in class_node.methods {
        draw_text_mut(img,black,textwidth,textline ,textsize, stdfont,  &*line);
        textline += newline;

    }

    let rect = Rect::at(pos1 as i32,pos2 as i32).of_size(rectsize_w ,rectsize_h);
    draw_hollow_rect_mut(   img , rect,black);

    let mut con_p = structures::Point{ x: (pos1+rectsize_w/2) as u32, y: pos2+rectsize_h};
    return con_p;
}


