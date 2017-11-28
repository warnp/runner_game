extern crate image;
extern crate glium;
extern crate notify;

use std::collections::HashMap;
use self::notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Cursor};
use std::error::Error;
use std::borrow::Borrow;
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ShaderCouple<'a> {
    pub vertex_shader: &'a str,
    pub pixel_shader: &'a str,
}

//#[derive(Debug)]
pub struct Shaders<'a> {
    display: &'a glium::Display,
    pub shaders_list: HashMap<&'a str, ShaderCouple<'a>>,
    compiled_shaders: HashMap<&'a str, Box<glium::program::Program>>,

    textures: Vec<&'a [u8]>,
}

impl<'a> Shaders<'a> {
    pub fn new(textures: Vec<&'a [u8]>, display: &'a glium::Display) -> Shaders<'a> {
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
                        vertex_shader: r#"
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

                                  uniform sampler2D diffuse_texture;
                                  uniform sampler2D light_texture;
                                  uniform sampler2D ui_texture;
                                  smooth in vec2 frag_texcoord;

                                  out vec4 color;

                                  void main(){

                                      vec3 difftex = texture(diffuse_texture, frag_texcoord).rgb;
                                      vec3 lighttex = texture(light_texture, frag_texcoord).rgb;
                                      vec3 uitex = texture(ui_texture, frag_texcoord).rgb;

                                        if(uitex.r == 0){
                                            color = vec4(difftex * lighttex, 1.0);

                                        }else{
                                            color = vec4(difftex * lighttex, 1.0) + vec4(uitex, 1.0);
                                        }
                                  }
                              "#,
                    });

        hash.insert("sprite_shader",
                    ShaderCouple {
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
//                                        color = texture(tex, vec3(v_tex_coords, float(v_tex_id)));
                                        color  = vec4(1.0,0.0,0.0,1.0);
                                    }
                                    "#,
                    });
        hash.insert("object_shader",
                    ShaderCouple {
                        vertex_shader:
                        r#"
                                    #version 150

                                    in vec3 position;
                                    in vec3 normal;

                                    uniform mat4 u_matrix;
                                    uniform mat4 u_world;

                                    out vec3 v_normal;

                                    void main(){
                                        v_normal = mat3(u_world) * normal;
                                        gl_Position = u_matrix * vec4(position,1.0);
                                    }
                                "#,
                        pixel_shader:
                        r#"
                                    #version 140


                                    in vec3 v_normal;
                                    in vec3 fragVert;
                                    uniform mat4 u_matrix;

                                    out vec4 diffuse_output;
                                    out vec4 normal_output;
                                    out vec4 position_output;

                                    void main(){
                                        diffuse_output = vec4(1.0,0.0,0.0,1.0);
                                        position_output = vec4(v_normal,1.0);
                                        normal_output = vec4(v_normal,1.0);
                                    }
                                    "#,
                    });
        hash.insert("light_shader",
                    ShaderCouple {
                        vertex_shader:
                        r#"
                                    #version 140

                                    in vec3 position;
                                    in vec2 tex_coords;

                                    out vec2 frag_coords;

                                    uniform mat4 matrix;

                                    void main(){
                                        frag_coords = tex_coords;
                                        gl_Position = matrix * vec4(position,1.0);
                                    }
                                "#,
                        pixel_shader:
                        r#"
                                    #version 140

                                    uniform sampler2D position_texture;
                                    uniform sampler2D normal_texture;
                                    uniform vec4 light_position;
                                    uniform vec3 light_color;
                                    uniform vec3 light_attenuation;
                                    uniform float light_radius;

                                    in vec2 frag_coords;

                                    out vec4 color;


                                    void main(){
                                        vec4 position = texture(position_texture, frag_coords);
                                        vec4 normal = texture(normal_texture, frag_coords);
                                        vec3 light_vector = light_position.xyz - position.xyz;
                                        float light_distance = abs(length(light_vector));
                                        vec3 normal_vector = normalize(normal.xyz);
                                        float diffuse = max(dot(normal_vector, light_vector), 0.0);
                                        if(diffuse > 0.0){
                                            float attenuation_factor = 1.0 /(
                                                light_attenuation.x +
                                                (light_attenuation.y * light_distance) +
                                                (light_attenuation.y * light_distance * light_distance)
                                            );
                                            attenuation_factor *= (1.0 - pow((light_distance /light_radius), 2.0));
                                            attenuation_factor = max(attenuation_factor, 0.0);
                                            diffuse *= attenuation_factor;
                                        }
                                        color  = vec4(light_color * diffuse,1.0);
                                    }
                                    "#,
                    });

        let mut hash_compiled = HashMap::new();
        //        let arc_display = Arc::new(display);
        //        let clone_display = arc_display.clone();

        thread::spawn(move || {
            let (tx, rx) = channel();
            let mut watcher: RecommendedWatcher = Watcher::new_raw(tx).unwrap();

            let context = glium::glutin::HeadlessRendererBuilder::new(1, 1).build().unwrap();

            let display = glium::HeadlessRenderer::new(context).unwrap();


            let files_watched = watcher.watch(Path::new("./content/shader_dev"), RecursiveMode::Recursive);

            loop {
                match rx.recv() {
                    Ok(notify::RawEvent { path: Some(path), op: Ok(op), cookie }) => {
                        println!("{:?} {:?} ({:?})", op, path, cookie);
                        let extension = path.extension();
                        if extension == Some(OsStr::new("shader")) {
                            let mut file = File::open(path.clone());
                            match file {
                                Ok(file) => {
                                    let mut buff = BufReader::new(file);
                                    let mut raw_shader_data = Vec::new();

                                    buff.read_to_end(&mut raw_shader_data);

                                    let string_shader_data = str::from_utf8(&raw_shader_data);

                                    match string_shader_data {
                                        Ok(string) => {
                                            if string != "" {
                                                let plop = String::from(string);
                                                let s: Vec<&str> = plop.split("//=================").collect();
                                                println!("Compilation shaders");
                                                let program = glium::Program::from_source(&display, s.get(0).unwrap(), s.get(1).unwrap(), None);
                                                match program {
                                                    Ok(prog) => {
                                                        println!("Compilation shaders réussi!");
                                                        println!("Création du fichier fx");
                                                        let binary = prog.get_binary();
                                                        let mut file = File::create("./content/shader/shader.fx").unwrap();
                                                        BufWriter::new(file).write_all(&binary.unwrap().content);
                                                        println!("Fichier fx créé");
                                                    }
                                                    Err(e) => println!("Error compile : {:#?}", e),
                                                }
                                            }
                                        }
                                        Err(e) => println!("Erreur dans le fichier")
                                    }
                                }
                                Err(e) => println!("Error open shader file : {}", e.description()),
                            }
                        }
                    }
                    Ok(event) => println!("broken event: {:?}", event),
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });


        for (name, s) in hash.iter() {
            hash_compiled.insert(*name, Box::new(glium::Program::from_source(display,
                                                                             s.vertex_shader,
                                                                             s.pixel_shader,
                                                                             None)
                .unwrap()));
        }

        Shaders {
            display: display,
            shaders_list: hash,
            compiled_shaders: hash_compiled,
            textures: textures,
        }
    }

    pub fn find_compiled_shader(&self)  -> Receiver<glium::program::Binary> {
        let (sender, receiver) = mpsc::channel();
        let thread = thread::spawn(move || {
            let (tx, rx) = channel();
            let mut watcher: RecommendedWatcher = Watcher::new_raw(tx).unwrap();

            let context = glium::glutin::HeadlessRendererBuilder::new(1, 1).build().unwrap();

            let display = glium::HeadlessRenderer::new(context).unwrap();


            let files_watched = watcher.watch(Path::new("./content/shader"), RecursiveMode::Recursive);
            loop {
                match rx.recv() {
                    Ok(notify::RawEvent { path: Some(path), op: Ok(op), cookie }) => {
                        println!("{:?} {:?} ({:?})", op, path, cookie);
                        let extension = path.extension();
                        if extension == Some(OsStr::new("fx")) {
                            let mut file = File::open(path.clone());

                            match file {
                                Ok(file) => {
                                    let mut buff = BufReader::new(file);
                                    let mut raw_shader_data = Vec::new();
                                    buff.read_to_end(&mut raw_shader_data);
                                    let binary = glium::program::Binary { format: 1, content: raw_shader_data };
                                    sender.send(binary).unwrap();
                                }
                                Err(e) => println!("Fail to open shader fx file")
                            }
                        }
                    }
                    Ok(event) => println!("broken event: {:?}", event),
                    Err(e) => println!("Fichier shader fx en erreur")
                }
            }
        });
        println!("plop");
        receiver
        //                let result_binary = receiver.recv().unwrap();
        //        println!("{:?}", result_binary.content);

        //            glium::Program::new(self.display, result_binary);
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
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            tex_vec.push(image);
        }

        glium::texture::Texture2dArray::new(display, tex_vec).unwrap()
    }
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
