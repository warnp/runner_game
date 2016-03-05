extern crate glium;

#[derive(Debug)]
pub struct InputManager;

impl InputManager {
    pub fn get_input(display: &glium::backend::glutin_backend::GlutinFacade) -> Vec<&str> {
        let mut result: Vec<&str> = vec![];
        // println!("{}", &display.poll_events());

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::Space)) => {
                    result.push("space_press")
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::Space)) => {
                    result.push("space_release")
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::A)) => {
                    result.push("a_press")
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::D)) => {
                    result.push("d_press")
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::P)) => {
                    result.push("p_press")
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::Escape)) => {
                    result.push("escape_press")
                }
                _ => (),
            }
            // println!("{:#?}", ev);
        }
        result
    }
}

// #[test]
// mod tests {
//
// }
