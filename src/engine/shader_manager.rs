extern crate image;
extern crate glium;
extern crate notify;

use std::collections::HashMap;
use self::notify::{RecommendedWatcher, Watcher, RecursiveMode};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::ffi::OsStr;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Cursor};
use std::error::Error;
use std::borrow::Borrow;
use std::str;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct ShaderCouple<'a> {
    pub vertex_shader: &'a str,
    pub pixel_shader: &'a str,
}

//#[derive(Debug)]
pub struct Shaders<'a> {
    display: &'a glium::Display,
    pub compiled_programms: HashMap<String, Box<glium::program::Program>>,

    textures: Vec<&'a [u8]>,
    receiver: Receiver<(String, glium::program::Binary)>,
}

impl<'a> Shaders<'a> {
    pub fn new(textures: Vec<&'a [u8]>, display: &'a glium::Display) -> Shaders<'a> {
        let compiled_programs = Shaders::get_shader_from_files(display);
        Shaders::reader_shaders_sources();


        Shaders {
            display: display,
            compiled_programms: compiled_programs,
            textures: textures,
            receiver: Shaders::find_compiled_shader(),
        }
    }


    pub fn update_program_list(&mut self) {
        match self.receiver.try_recv() {
            Ok(t) => {
                self.insert_program(t);
            }
            Err(e) => ()
        };
    }

    fn insert_program(&mut self, t: (String, glium::program::Binary)) -> () {
        if t.1.content.len() > 0 {
            let b = glium::program::Binary { content: t.1.content, format: 1 };
            let mut shader_name = t.0;
            shader_name.push_str("_shader");

            self.compiled_programms.insert(shader_name, Box::new(glium::Program::new(self.display,
                                                                                     glium::program::ProgramCreationInput::Binary {
                                                                                         data: b,
                                                                                         outputs_srgb: true,
                                                                                         uses_point_size: true,
                                                                                     }).unwrap()));
            println!("Shader récupéré");
            println!("Shader pool : {:#?}", self.compiled_programms);
        }
    }

    fn find_compiled_shader() -> Receiver<(String, glium::program::Binary)> {
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
                        let shader_name = path.file_stem().unwrap().to_str().unwrap().to_string();
                        if extension == Some(OsStr::new("fx")) {
                            let mut file = File::open(path.clone());

                            match file {
                                Ok(file) => {
                                    let mut buff = BufReader::new(file);
                                    let mut raw_shader_data = Vec::new();
                                    buff.read_to_end(&mut raw_shader_data);
                                    let binary = glium::program::Binary { format: 1, content: raw_shader_data };
                                    sender.send((shader_name, binary)).unwrap();
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
        receiver
    }

    pub fn get_shader_from_files(display: &glium::Display) -> HashMap<String, Box<glium::Program>> {
        let mut result: HashMap<String, Box<glium::Program>> = HashMap::new();
        let paths = fs::read_dir("./content/shader/").unwrap();
        for path_dir in paths {
            match path_dir {
                Ok(dir) => {
                    let path = dir.path();
                    let extension = path.extension();
                    let mut shader_name = path.file_stem().unwrap().to_str().unwrap().to_string();
                    if extension == Some(OsStr::new("fx")) {
                        let mut file = File::open(path.clone());

                        match file {
                            Ok(file) => {
                                let mut buff = BufReader::new(file);
                                let mut raw_shader_data = Vec::new();
                                buff.read_to_end(&mut raw_shader_data);
                                let binary = glium::program::Binary { format: 1, content: raw_shader_data };
                                if binary.content.len() > 0 {
                                    let b = glium::program::Binary { content: binary.content, format: 1 };
                                    shader_name.push_str("_shader");

                                    result.insert(shader_name, Box::new(glium::Program::new(display,
                                                                                            glium::program::ProgramCreationInput::Binary {
                                                                                                data: b,
                                                                                                outputs_srgb: true,
                                                                                                uses_point_size: true,
                                                                                            }).unwrap()));
                                    println!("Shader récupéré");
                                    println!("Shader pool : {:#?}", result);
                                }
                            }
                            Err(e) => {
                                println!("Fail to open shader fx file");
                            }
                        }
                    }
                }
                Err(t) => {
                    println!("Failed to read shader dir");
                }
            }
        }
        result
    }

    fn set_image(&self, texture: &'a [u8]) -> image::ImageResult<image::DynamicImage> {
        image::load(Cursor::new(texture), image::PNG)
    }

    pub fn get_texture_array(&self,
                             display: &glium::Display)
                             -> glium::texture::Texture2dArray {
        let mut tex_vec = Vec::new();

        for tex in &self.textures {
            let image = self.set_image(tex).unwrap().to_rgba();
            let image_dimensions = image.dimensions();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            tex_vec.push(image);
        }

        glium::texture::Texture2dArray::new(display, tex_vec).unwrap()
    }

    fn reader_shaders_sources() {
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
                        let shader_name = path.file_stem().unwrap().to_str().unwrap().to_string();
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
                                                let file_content = String::from(string);
                                                let shader_source: Vec<&str> = file_content.split("//=================").collect();
                                                println!("Compilation shaders");
                                                let program = glium::Program::from_source(&display, shader_source.get(0).unwrap(), shader_source.get(1).unwrap(), None);
                                                match program {
                                                    Ok(prog) => {
                                                        println!("Compilation shaders réussi!");
                                                        println!("Création du fichier fx");
                                                        let binary = prog.get_binary();
                                                        let mut fx_file_path = "./content/shader/".to_string();
                                                        fx_file_path.push_str(&shader_name);
                                                        let mut fx_file_path = fx_file_path.to_string();
                                                        fx_file_path.push_str(".fx");
                                                        let mut file = File::create(fx_file_path).unwrap();
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
    }
}


#[cfg(test)]
mod shader_manager_tests {
    use super::*;

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
