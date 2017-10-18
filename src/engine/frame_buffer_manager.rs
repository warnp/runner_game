extern crate glium;

pub struct FrameBufferManager<'a>{
    frame_texture: glium::texture::Texture2d,
    frame_buffer: Option<glium::framebuffer::SimpleFrameBuffer<'a>>,

}

impl<'a> FrameBufferManager<'a>{


    pub fn new(display: &glium::Display) -> FrameBufferManager<'a>{
        let frame_texture = glium::texture::Texture2d::empty_with_format(display,
             glium::texture::UncompressedFloatFormat::F32F32F32F32,
              glium::texture::MipmapsOption::NoMipmap,
              800,
              600).unwrap();


        FrameBufferManager {
            frame_texture: frame_texture,
            frame_buffer: None,
        }
    }

    pub fn init_frame_buffer(&'a mut self, display: &glium::Display){
        let mut frame_buffer = glium::framebuffer::SimpleFrameBuffer::new(display, &self.frame_texture).unwrap();
        self.frame_buffer = Some(frame_buffer);
    }

    pub fn draw(&self){

    }
}
