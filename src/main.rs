extern crate xml;

use std::env;
use std::fs::{ self, File };
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

#[derive(Clone, Copy, Debug)]
struct Position {
  x: f64,
  y: f64,
}

impl Default for Position {
  fn default() -> Self {
    Position {
      x: 0.0,
      y: 0.0,
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct Circle {
  position: Position,
  radius: f64,
}

impl Default for Circle {
  fn default() -> Self {
    Circle {
      position: Position::default(),
      radius: 0.0,
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct Ellipse {
  position: Position,
  radius_x: f64,
  radius_y: f64,
}

impl Default for Ellipse {
  fn default() -> Self {
    Ellipse {
      position: Position::default(),
      radius_x: 0.0,
      radius_y: 0.0,
    }
  }
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
            ".movePointerTo([{}, {}]) // MoveAbsolute",
            self.current_point.x,
            -self.current_point.y,
          );
        }
      },
      Command::MoveRelative => {
        for args in self.values.chunks(2) {
          self.current_point.x += args[0];
          self.current_point.y += args[1];
          
          println!(
            ".movePointerTo([{}, {}]) // MoveRelative",
            self.current_point.x,
            -self.current_point.y,
          );
        }
      },
      Command::LineAbsolute => {
        for args in self.values.chunks(2) {
          self.current_point.x = args[0];
          self.current_point.y = args[1];
          
          println!(
            ".lineTo({}, {}) // LineAbsolute",
            self.current_point.x,
            -self.current_point.y,
          );
        }
      },
      Command::LineRelative => {
        for args in self.values.chunks(2) {
          self.current_point.x += args[0];
          self.current_point.y += args[1];
          
          println!(
            ".line({}, {}) // LineRelative",
            self.current_point.x,
            -self.current_point.y,
          );
        }
      },
      Command::HorizontalLineAbsolute => {
        for args in self.values.chunks(2) {
          self.current_point.x = args[0];
          
          println!(
            ".hLineTo({}) // HorizontalLineAbsolute",
            self.current_point.x
          );
        }
      },
      Command::HorizontalLineRelative => {
        for args in self.values.chunks(2) {
          self.current_point.x += args[0];
          
          println!(
            ".hLine({}) // HorizontalLineRelative",
            self.current_point.x
          );
        }
      },
      Command::VerticalLineAbsolute => {
        for args in self.values.chunks(2) {
          self.current_point.y = args[0];
          
          println!(
            ".vLineTo({}) // VerticalLineAbsolute",
            -self.current_point.y
          );
        }
      },
      Command::VerticalLineRelative => {
        for args in self.values.chunks(2) {
          self.current_point.y += args[0];
          
          println!(
            ".vLine({}) // VerticalLineRelative",
            -self.current_point.y
          );
        }
      },
      Command::QuadraticBezierAbsolute => {
        for args in self.values.chunks(4) {
          println!(
            ".quadraticBezierCurveTo([{}, {}], [{}, {}]), // QuadraticBezierAbsolute",
            args[2],
            -args[3],
            args[0],
            -args[1],
          );

          self.current_point.x = args[2];
          self.current_point.y = args[3];
        }
      },
      Command::QuadraticBezierRelative => {
        for args in self.values.chunks(4) {
          if args.len() != 4 {
            // Should never happen but does because of how other software exports SVGs.
            return;
          }
          
          println!(
            ".quadraticBezierCurveTo([{}, {}], [{}, {}]), // QuadraticBezierRelative",
            self.current_point.x + args[2],
            -(self.current_point.y + args[3]),
            self.current_point.x + args[0],
            -(self.current_point.y + args[1]),
          );

          self.current_point.x += args[2];
          self.current_point.y += args[3];
        }
      },
      Command::QuadraticBezierSmoothAbsolute => { },
      Command::QuadraticBezierSmoothRelative => { },
      Command::CubicBezierAbsolute => {
        for args in self.values.chunks(6) {
          println!(
            ".cubicBezierCurveTo([{}, {}], [{}, {}], [{}, {}]) // CubicBezierAbsolute",
            args[4],
            -args[5],
            args[0],
            -args[1],
            args[2],
            -args[3],
          );

          self.current_point.x = args[4];
          self.current_point.y = args[5];
        }
      },
      Command::CubicBezierRelative => {
        for args in self.values.chunks(6) {
          if args.len() != 6 {
            // Should never happen but does because of how other software exports SVGs.
            return;
          }
          
          println!(
            ".cubicBezierCurveTo([{}, {}], [{}, {}], [{}, {}]) // CubicBezierRelative",
            self.current_point.x + args[4],
            -(self.current_point.y + args[5]),
            self.current_point.x + args[0],
            -(self.current_point.y + args[1]),
            self.current_point.x + args[2],
            -(self.current_point.y + args[3]),
          );

          self.current_point.x += args[4];
          self.current_point.y += args[5];
        }
      },
      Command::CubicBezierSmoothAbsolute => { },
      Command::CubicBezierSmoothRelative => { },
      Command::EllipticalArcAbsolute => { },
      Command::EllipticalArcRelative => { },
      Command::StopAbsolute => {
        println!(" // StopAbsolute");
      },
      Command::StopRelative=> {
        println!(" // StopRelative");
      },
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
  let extra_curv = fs::read_to_string("extra.rcad").unwrap();

  let args: Vec<String> = env::args().collect();
  let drawing_svg = File::open(args[1].clone()).unwrap();
  let drawing_svg = BufReader::new(drawing_svg);
  let parser = EventReader::new(drawing_svg);

  println!("const svg = draw()");
  
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
                ".fuse(drawCircle({}).translate({}, {}))",
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
                ".fuse(drawEllipse({}, {}).translate({}, {}))",
                ellipse.radius_x,
                ellipse.radius_y,
                ellipse.position.x,
                -ellipse.position.y,
              );
            },

            "path" => {
              println!(".fuse(draw()");
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
                          state.push_value();
                        },
                        _ => {}
                      }
                      
                    } // for

                    // It's possible we stopped without processing the last command
                    // because some programs generate without Z or z command.
                    // We know this if there is still data in the value buffer.
                    // Pushing a fake Z or z command will resolve the issue.
                    if state.value_buffer.len() > 0 {
                      state.push_value();
                      state.push_command(Command::StopAbsolute);
                    }
                  },
                  _ => {}
                }
              }
              println!(")");
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
  
  println!(".done().sketchOnPlane(new Plane('XY'));");
}
