extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

#[derive(Clone, Copy)]
struct Position {
  x: f32,
  y: f32,
}

#[derive(Clone, Copy)]
struct Circle {
  position: Position,
  radius: f32,
}

#[derive(Clone, Copy)]
struct Ellipse {
  position: Position,
  radius_x: f32,
  radius_y: f32,
}

fn main() {
  let file = File::open("file.xml").unwrap();
  let file = BufReader::new(file);
  let parser = EventReader::new(file);

  let position_global = Position {
    x: 0.0,
    y: 0.0,
  };

  println!("union [");
  
  for e in parser {
    match e {
        Ok(XmlEvent::StartElement { name, attributes, .. }) => {
          match name.local_name.as_str() {
            "circle" => {
              let mut circle = Circle {
                position: position_global,
                radius: 0.0,
              };
              for attr in attributes {
                match attr.name.local_name.as_str() {
                  "cx" => circle.position.x = attr.value.parse::<f32>().unwrap(),
                  "cy" => circle.position.y = attr.value.parse::<f32>().unwrap(),
                  "r" => circle.radius = attr.value.parse::<f32>().unwrap(), 
                  _ => {}
                }
              }
              println!(
                "circle {} >> move [{}, {}, 0],",
                circle.radius,
                circle.position.x,
                circle.position.y,
              );
            },
            
            "ellipse" => {
              let mut ellipse = Ellipse {
                position: position_global,
                radius_x: 0.0,
                radius_y: 0.0,
              };
              for attr in attributes {
                match attr.name.local_name.as_str() {
                  "cx" => ellipse.position.x = attr.value.parse::<f32>().unwrap(),
                  "cy" => ellipse.position.y = attr.value.parse::<f32>().unwrap(),
                  "rx" => ellipse.radius_x = attr.value.parse::<f32>().unwrap(), 
                  "ry" => ellipse.radius_y = attr.value.parse::<f32>().unwrap(), 
                  _ => {}
                }
              }
              println!(
                "ellipse [ {}, {} ] >> move [{}, {}, 0],",
                ellipse.radius_x,
                ellipse.radius_y,
                ellipse.position.x,
                ellipse.position.y,
              );
            },

            "path" => {
              let mut path = Path { commands: [] };
              match attr.name.local_name.as_str() {
                "d" => {
                  for token in attr.value.split(' ') {
                    match token {
                      "M" => (),
                      "m" => (),
                      "L" => (),
                      "l" => (),
                      "H" => (),
                      "h" => (),
                      "V" => (),
                      "v" => (),
                      "C" => (),
                      "c" => (),
                      "S" => (),
                      "s" => (),
                      "Q" => (),
                      "q" => (),
                      "T" => (),
                      "t" => (),
                      "A" => (),
                      "a" => (),
                      "Z" => (),
                      "z" => (),
                    }
                  }
                },
                _ => {}
              }
            },
            _ => {}
          }
        }
        Ok(XmlEvent::EndElement { name: _ }) => {
        }
        Err(e) => {
          println!("Error: {}", e);
          break;
        }
        _ => {}
    }
  }
  
  println!("]");
}
