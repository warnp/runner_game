use std::collections::HashMap;
use std::io::Cursor;
extern crate image;

extern crate glium;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ShaderCouple<'a> {
    pub vertex_shader: &'a str,
    pub pixel_shader: &'a str,
}

#[derive(Debug)]
pub struct Shaders<'a> {
    pub shaders_list: HashMap<&'a str, ShaderCouple<'a>>,
    compiled_shaders: HashMap<&'a str, Box<glium::program::Program>>,

    textures: Vec<&'a [u8]>,
}

impl<'a> Shaders<'a> {
    pub fn new(textures: Vec<&'a [u8]>, display: &glium::Display) -> Shaders<'a> {

        let mut hash = HashMap::new();

        hash.insert("simple_shader",
                    ShaderCouple {
                        vertex_shader: r#"
            #version 140

            in vec3 position;
            in vec3 normal;
            in vec4 color;
            in vec2 tex_coords;
            in uint i_tex_id;

            out vec4 colorV;
            out vec3 v_normal;
            out vec2 v_tex_coords;
            flat out uint v_tex_id;

            uniform mat4 matrix;

            void main(){
                // colorV = color;
                v_tex_coords = tex_coords;
                gl_Position = matrix * vec4(position,1.0);
                v_tex_id = i_tex_id;
            }
            "#,

                        pixel_shader: r#"
            #version 140

            in vec4 colorV;
            in vec2 v_tex_coords;
            flat in uint v_tex_id;

            out vec4 color;

            // uniform sampler2D tex;
            uniform sampler2DArray tex;

            void main(){
                color = texture(tex, vec3(v_tex_coords, float(v_tex_id)));
            }
            "#,
                    });
        hash.insert("screen_shader",
                        ShaderCouple {
                            vertex_shader:r#"
                                  #version 140

                                  in vec3 position;
                                  in vec2 tex_coords;

                                  uniform mat4 matrix;

                                  smooth out vec2 frag_texcoord;

                                  void main(){
                                      frag_texcoord = tex_coords;
                                      gl_Position = matrix * vec4(position,1.0);
                                  }
                              "#,

                              // fragment shader
                              pixel_shader: r#"
                                  #version 140

                                  uniform sampler2D ui_texture;
                                  smooth in vec2 frag_texcoord;

                                  out vec4 color;

                                //   float kernel[9] = float[](
                                //       1,1,1,
                                //       1,-8,1,
                                //       1,1,1
                                //   );
                                  //
                                //   const float offset = 1.0/300.0;

                                  void main(){
                                    //   vec2 offsets[9] = vec2[](
                                    //         vec2(-offset, offset),  // top-left
                                    //         vec2(0.0f,    offset),  // top-center
                                    //         vec2(offset,  offset),  // top-right
                                    //         vec2(-offset, 0.0f),    // center-left
                                    //         vec2(0.0f,    0.0f),    // center-center
                                    //         vec2(offset,  0.0f),    // center-right
                                    //         vec2(-offset, -offset), // bottom-left
                                    //         vec2(0.0f,    -offset), // bottom-center
                                    //         vec2(offset,  -offset)  // bottom-right
                                    //     );

                                      vec3 tex = texture(ui_texture, frag_texcoord).rgb;
                                    //   vec3 sample[9];
                                    //   for(int i = 0; i < 9; i++){
                                    //       sample[i] = vec3(texture(ui_texture, frag_texcoord + offsets[i]));
                                    //   }
                                    //   vec3 col = vec3(0.0);
                                    //   for(int i = 0; i < 9; i++){
                                    //       col += sample[i] * kernel[i];
                                    //   }
                                      color = vec4(tex, 1.0);
                                  }
                              "#,
                        });

            hash.insert("sprite_shader",
                            ShaderCouple{
                                vertex_shader:
                                 r#"
                                    #version 140

                                    in vec3 position;
                                    in vec3 normal;
                                    in vec4 color;
                                    in vec2 tex_coords;
                                    in uint i_tex_id;

                                    out vec4 colorV;
                                    out vec3 v_normal;
                                    out vec2 v_tex_coords;
                                    flat out uint v_tex_id;

                                    uniform mat4 matrix;

                                    void main(){
                                    v_tex_coords = tex_coords;
                                    gl_Position = matrix * vec4(position,1.0);
                                    v_tex_id = i_tex_id;
                                    }
                                "#,
                            pixel_shader:
                                r#"
                                    #version 140

                                    in vec4 colorV;
                                    in vec2 v_tex_coords;
                                    flat in uint v_tex_id;

                                    out vec4 color;

                                    uniform sampler2DArray tex;

                                    void main(){
                                        //color = texture(tex, vec3(v_tex_coords, float(v_tex_id)));
                                        color  = vec4(1.0,0.0,0.0,1.0);
                                    }
                                    "#,
                            });
        hash.insert("object_shader",
                    ShaderCouple{
                        vertex_shader:
                        r#"
                                    #version 150

                                    in vec3 position;
                                    in vec3 normal;

                                    out vec3 v_normal;

                                    uniform mat4 u_matrix;

                                    void main(){
                                        v_normal = transpose(inverse(mat3(u_matrix))) * normal;
                                        gl_Position = u_matrix * vec4(position,1.0);
                                    }
                                "#,
                        pixel_shader:
                        r#"
                                    #version 140

                                    const vec3 light = vec3(-10.0, 10.0, 0.0);

                                    in vec3 v_normal;

                                    void main(){
                                        //color = texture(tex, vec3(v_tex_coords, float(v_tex_id)));
                                        float brightness = dot(normalize(v_normal), normalize(light));

                                        gl_FragColor  = mix(vec4(0.6,0.0,0.0,1.0), vec4(1.0,0.0,0.0,1.0), brightness);
//                                        gl_FragColor = vec4(1.0,0.0,0.0,1.0);
                                    }
                                    "#,
                    });
        let mut hash_compiled = HashMap::new();

        for (name, s) in hash.iter() {
            hash_compiled.insert(*name, Box::new(glium::Program::from_source(display,
                                                 s.vertex_shader,
                                                 s.pixel_shader,
                                                 None)
                         .unwrap()));
        }

        Shaders {
            shaders_list: hash,
            compiled_shaders: hash_compiled,
            textures: textures,
        }

    }

    pub fn compile_shaders(&mut self, display: &glium::Display) {
        for (name, s) in self.shaders_list.iter() {
            self.compiled_shaders.insert(name,
                                         Box::new(glium::Program::from_source(display,
                                                                              s.vertex_shader,
                                                                              s.pixel_shader,
                                                                              None)
                                                      .unwrap()));
        }

    }

    pub fn get_compiled_shader(&mut self, shader_name: &'a str) -> glium::program::Program {
        *self.compiled_shaders.remove(&shader_name).unwrap()
    }

    fn set_image(&self, texture: &'a [u8]) -> image::ImageResult<image::DynamicImage> {

        image::load(Cursor::new(texture), image::PNG)

    }

    pub fn get_texture_array(&self,
                             display: &glium::Display)
                             -> glium::texture::Texture2dArray {
        let mut tex_vec = Vec::new();

        for tex in &self.textures {
//            let image = self.set_image(tex).unwrap().to_rgba();
            let image = self.set_image(tex).unwrap().to_rgba();
            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions );
            tex_vec.push(image);
        }

        glium::texture::Texture2dArray::new(display, tex_vec).unwrap()
    }


    // TODO implémenter get_binary() pour sauvegarder le shader
}


#[cfg(test)]
mod shader_manager_tests {
    use super::*;

    use glium::DisplayBuild;

    extern crate glium;

    #[test]
    #[ignore]
    fn should_return_a_shader() {

        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

      let mut shader = Shaders::new(vec![&include_bytes!("../../content/NatureForests.png")[..],
                                             &include_bytes!("../../content/11532.png")[..]], &display);
        let lst_shaders = shader.get_compiled_shader("toto");
        // assert!(lst_shaders.len() == 1);
        // Shall not pass for the moment, need to find a property to evaluate
    }

    #[cfg(not(feature = "integration"))]
    #[test]
    fn should_get_texture_array() {

        let display = glium::glutin::WindowBuilder::new()
                          .with_visibility(false)
                          .build_glium()
                          .unwrap();

      let mut shader = Shaders::new(vec![&include_bytes!("../../content/NatureForests.png")[..],
                                         &include_bytes!("../../content/11532.png")[..]], &display);

        let lst_shaders = shader.get_texture_array(&display);
        assert!(lst_shaders.get_array_size() == Some(2));
    }
}
