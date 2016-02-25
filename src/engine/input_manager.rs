extern crate glium;

pub struct InputManager;

impl InputManager {
    pub fn get_input(display: &glium::backend::glutin_backend::GlutinFacade) -> &str {
        let mut result = "";
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::Space)) => {
                    result = "space_press"
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Released,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::Space)) => {
                    result = "space_release"
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::A)) => {
                    result = "a_press"
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::D)) => {
                    result = "d_press"
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::P)) => {
                    result = "p_press"
                }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(glium::glutin::VirtualKeyCode::Escape)) => {
                    result = "escape_press"
                }
                _ => result = "",
            }
        }
        result
    }
}

// #[test]
// mod tests {
//
// }
