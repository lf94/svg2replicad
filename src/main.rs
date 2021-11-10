extern crate xml;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

#[derive(Clone, Copy, Debug)]
struct Position {
  x: f64,
  y: f64,
}

#[derive(Clone, Copy, Debug)]
struct Circle {
  position: Position,
  radius: f64,
}

#[derive(Clone, Copy, Debug)]
struct Ellipse {
  position: Position,
  radius_x: f64,
  radius_y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Command {
  NotSet,
  MoveAbsolute,
  MoveRelative,
  LineAbsolute,
  LineRelative,
  HorizontalLineAbsolute,
  HorizontalLineRelative,
  VerticalLineAbsolute,
  VerticalLineRelative,
  QuadraticBezierAbsolute,
  QuadraticBezierRelative,
  QuadraticBezierSmoothAbsolute,
  QuadraticBezierSmoothRelative,
  CubicBezierAbsolute,
  CubicBezierRelative,
  CubicBezierSmoothAbsolute,
  CubicBezierSmoothRelative,
  EllipticalArcAbsolute,
  EllipticalArcRelative,
  StopAbsolute,
  StopRelative,
}

#[derive(Clone, Debug)]
struct PathState {
  command: Command,
  values: Vec<f64>,
  value_buffer: String,

  // Handles the case of whitespace preceding a command, because detecting a new
  // command will also cause the value to be pushed to the values stack.
  is_value_already_pushed: bool,

  // See the terminology at https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#moveto_path_commands
  current_point: Position,
}

impl PathState {
  fn push_value(&mut self) {
    // Don't push anything if there is nothing buffered.
    if self.value_buffer.len() == 0 {
      return;
    }
      
    let value = self.value_buffer.parse::<f64>().unwrap();
    self.values.push(value);
    self.value_buffer = "".to_string();
  }

  fn push_command(&mut self, command: Command) {
    if self.is_value_already_pushed == false {
      self.push_value();
      self.is_value_already_pushed = true;
    }

    self.handle_command();

    let current_point = self.current_point;
    *self = PathState::default();
    self.command = command;
    self.current_point = current_point;
  }

  fn handle_command(&mut self) {
    // Do things based on the command
    // println!("{:?}", self);
    match self.command {
      Command::NotSet => { /* Do nothing when the command is not set. */ },
      Command::MoveAbsolute => {
        for args in self.values.chunks(2) {
          self.current_point.x = args[0];
          self.current_point.y = args[1];
          
          println!(
            "  [[{}, {}]], // MoveAbsolute",
            self.current_point.x,
            self.current_point.y,
          );
        }
      },
      Command::MoveRelative => {
        for args in self.values.chunks(2) {
          self.current_point.x += args[0];
          self.current_point.y += args[1];
          
          println!(
            "  [[{}, {}]], // MoveRelative",
            self.current_point.x,
            self.current_point.y,
          );
        }
      },
      Command::LineAbsolute => { },
      Command::LineRelative => { },
      Command::HorizontalLineAbsolute => { },
      Command::HorizontalLineRelative => { },
      Command::VerticalLineAbsolute => { },
      Command::VerticalLineRelative => { },
      Command::QuadraticBezierAbsolute => { },
      Command::QuadraticBezierRelative => { },
      Command::QuadraticBezierSmoothAbsolute => { },
      Command::QuadraticBezierSmoothRelative => { },
      Command::CubicBezierAbsolute => { },
      Command::CubicBezierRelative => {
        for args in self.values.chunks(6) {
          println!(
            "  bezier4 [[{}, {}], [{}, {}], [{}, {}]] 1.0, // CubicBezierRelative",
            self.current_point.x + args[0],
            self.current_point.y + args[1],
            self.current_point.x + args[2],
            self.current_point.y + args[3],
            self.current_point.x + args[4],
            self.current_point.y + args[5],
          );

          self.current_point.x += self.values[4];
          self.current_point.y += self.values[5];
        }
      },
      Command::CubicBezierSmoothAbsolute => { },
      Command::CubicBezierSmoothRelative => { },
      Command::EllipticalArcAbsolute => { },
      Command::EllipticalArcRelative => { },
      Command::StopAbsolute => { /* Used to stop path processing. */ },
      Command::StopRelative => { /* Used to stop path processing. */ },
    }
  }
}

impl Default for PathState {
  fn default() -> Self {
    PathState {
      command: Command::NotSet,
      values: Vec::new(),
      value_buffer: "".to_string(),
      is_value_already_pushed: false,
      current_point: Position {
        x: 0.0,
        y: 0.0,
      },
    }
  }
}

fn main() {
  let extraCurv = fs::read_to_string("extra.curv").unwrap();
  
  let drawingSvg = File::open("drawing.svg").unwrap();
  let drawingSvg = BufReader::new(drawingSvg);
  let parser = EventReader::new(drawingSvg);

  println!("{}", extraCurv);

  println!("polygon << concat [");
  
  for e in parser {
    match e {
        Ok(XmlEvent::StartElement { name, attributes, .. }) => {
          match name.local_name.as_str() {
            "circle" => {
              let mut circle = Circle::default();
              for attr in attributes {
                match attr.name.local_name.as_str() {
                  "cx" => circle.position.x = attr.value.parse::<f64>().unwrap(),
                  "cy" => circle.position.y = attr.value.parse::<f64>().unwrap(),
                  "r" => circle.radius = attr.value.parse::<f64>().unwrap(), 
                  _ => {}
                }
              }
              println!(
                "circle {} >> move [{}, {}, 0],",
                circle.radius,
                circle.position.x,
                -circle.position.y,
              );
            },
            
            "ellipse" => {
              let mut ellipse = Ellipse::default();
              for attr in attributes {
                match attr.name.local_name.as_str() {
                  "cx" => ellipse.position.x = attr.value.parse::<f64>().unwrap(),
                  "cy" => ellipse.position.y = attr.value.parse::<f64>().unwrap(),
                  "rx" => ellipse.radius_x = attr.value.parse::<f64>().unwrap(), 
                  "ry" => ellipse.radius_y = attr.value.parse::<f64>().unwrap(), 
                  _ => {}
                }
              }
              println!(
                "ellipse [ {}, {} ] >> move [{}, {}, 0],",
                ellipse.radius_x,
                ellipse.radius_y,
                ellipse.position.x,
                -ellipse.position.y,
              );
            },

            "path" => {
              for attr in attributes {
                match attr.name.local_name.as_str() {
                  "d" => {
                    let mut state = PathState::default();
                    // To handle the initial Command::NotSet, which has no values.
                    state.is_value_already_pushed = true;
                    
                    for char in attr.value.chars() {
                      match char {
                        'M' => state.push_command(Command::MoveAbsolute),
                        'm' => state.push_command(Command::MoveRelative),
                        'L' => state.push_command(Command::LineAbsolute),
                        'l' => state.push_command(Command::LineRelative),
                        'H' => state.push_command(Command::HorizontalLineAbsolute),
                        'h' => state.push_command(Command::HorizontalLineRelative),
                        'V' => state.push_command(Command::VerticalLineAbsolute),
                        'v' => state.push_command(Command::VerticalLineRelative),
                        'C' => state.push_command(Command::CubicBezierAbsolute),
                        'c' => state.push_command(Command::CubicBezierRelative),
                        'S' => state.push_command(Command::CubicBezierSmoothAbsolute),
                        's' => state.push_command(Command::CubicBezierSmoothRelative),
                        'Q' => state.push_command(Command::QuadraticBezierAbsolute),
                        'q' => state.push_command(Command::QuadraticBezierRelative),
                        'T' => state.push_command(Command::QuadraticBezierSmoothAbsolute),
                        't' => state.push_command(Command::QuadraticBezierSmoothRelative),
                        'A' => state.push_command(Command::EllipticalArcAbsolute),
                        'a' => state.push_command(Command::EllipticalArcRelative),
                        'Z' => state.push_command(Command::StopAbsolute),
                        'z' => state.push_command(Command::StopRelative),
                        ',' => {
                          state.push_value();
                        },
                        '-' => {
                          state.push_value();
                          state.value_buffer.push(char);
                        },
                        ' ' => {
                          state.push_value();
                          state.is_value_already_pushed = true;
                        },
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                          state.value_buffer.push(char);
                        },
                        _ => { /* Ignore all other characters */ }
                      }

                      match state.command {
                        Command::StopAbsolute | Command::StopRelative => {
                          // Do not continue looping through the path string.
                          // println!("Stop");
                          break;
                        },
                        _ => {}
                      }
                      
                    } // for
                  },
                  _ => {}
                }
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
