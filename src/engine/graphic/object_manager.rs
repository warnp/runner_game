extern crate notify;

extern crate cgmath;

use engine::graphic::model::{StaticMesh, Lod};
use engine::graphic::vertex::{Vertex};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::thread;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use self::notify::{RecommendedWatcher, Watcher, RecursiveMode};
use engine::graphic::importer::obj_importer::ObjImporter;
use self::cgmath::{Matrix4, Vector3, Vector4};
use self::cgmath::prelude::*;
use engine::graphic::RESOURCES_PATH;

extern crate glium;

#[derive(Debug)]
pub struct ObjectManager {
    receiver: Receiver<(String, BufReader<File>)>,
    pub available_models: Vec<StaticMesh>,
    pub models_loader_receiver: Vec<Receiver<(String, (Vec<Vertex>, Vec<u16>))>>,
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        ObjectManager {
            receiver: ObjectManager::realtime_reload_objects(),
            available_models: vec![],
            models_loader_receiver: vec![],
        }
    }

    pub fn update_lod(&mut self, camera_position: Vector3<f32>) {
//        for el in &self.available_models.borrow().into_iter() {
//el.
//        }
    }

    pub fn update_loaded_model_list(&mut self, camera_position: Vector4<f32>, model_to_load_names: Vec<String>) {
        println!("look for mesh");
//Trouver l'ensemble des mesh à afficher déjà présent dans le buffer
        let mut model_to_load = &mut self.available_models.clone();
        println!("size {}", model_to_load.len());

        for model in model_to_load.iter_mut() {
            println!("Begin loading model {}", model.name);
            let (sender, receiver) = mpsc::channel();
            self.models_loader_receiver.push(receiver);
            //TODO Gérer les niveau de details plus haut
            let object_name = &model.name;

            let mut lod_level_to_load = -1;
            let mesh_name = {
                let distance = camera_position.distance(model.matrix.row(2).clone());
                println!("distance {}", distance);
                if distance > 200.0 {
                    lod_level_to_load = 1i8;
                    if let Some(lod_name) = model.lods.get(&1i8) {
                        lod_name.clone().mesh_name
                    }else{
                        "".to_string()
                    }
                } else {
                    lod_level_to_load = 0i8;
                    if let Some(lod_name) = model.lods.get(&0i8) {
                        lod_name.clone().mesh_name
                    }else{
                        "".to_string()
                    }
                }
            };

            if mesh_name == "".to_string() {
                continue;
            }

            println!("lod level {}", lod_level_to_load);
            if model.actual_lod == lod_level_to_load {
                println!("No need to reload model");
                return;
            }

            model.actual_lod = lod_level_to_load;

            let path_string = format!("{}/objects/{}/{}", RESOURCES_PATH, object_name, mesh_name);


            println!("model to be loaded {}", path_string);

            thread::spawn(move || {
                println!("begin thread loading");
                let model_path = Path::new(&path_string);

                let extension = model_path.extension();
                let object_name = model_path.file_stem().unwrap().to_str().unwrap().to_string();
                println!("file extension {:#?}", extension);

                if extension == Some(OsStr::new("obj")) {
                    let mut file = File::open(model_path.clone());

                    match file {
                        Ok(file) => {
                            let mut buff = BufReader::new(file);
                            let obj_content = ObjImporter::import(buff);
                            println!("Obj loaded");
                            sender.send((object_name, obj_content)).unwrap();
                        }
                        Err(e) => println!("Fail to open obj file")
                    };
                };
            });
        }

        self.available_models = model_to_load.to_vec();
    }

    pub fn load_models_into_buffer(&mut self) {
        for receiver in &self.models_loader_receiver {
            match receiver.try_recv() {
                Ok(t) => {
                    let mut cubes = self.available_models.iter_mut()
                        .filter(|x| *x.name == t.0.split('_').collect::<Vec<&str>>().get(0).unwrap().to_string())
                        .collect::<Vec<&mut StaticMesh>>();

                    let mut model = match cubes.get_mut(0) {
                        Some(m) => m,
                        None => return
                    };

                    model.vertices = t.1 .0;
                    model.indices = t.1 .1;
//                    model.normals = t.1 .2;

//                    model.matrix = Matrix4::identity();
                }
                Err(e) => {
//                    println!("Erreur chargement modèle {:#?}", e)
                }
            };
        }
    }

    pub fn update_realtime_objects_list(&mut self) {
        match self.receiver.try_recv() {
            Ok(mut t) => {
                let mut line = String::new();
                println!("hi there {}", t.1.read_line(&mut line).unwrap());
                //TODO compute mesh (async?)
            }
            Err(e) => {}
        }
    }

    pub fn preload_object_list(&mut self) -> Result<(), Error> {
        let p = RESOURCES_PATH.to_string() + "/objects";
        let path = Path::new(&p);
        if path.is_dir() {
            let content = fs::read_dir(path)?;
            for entry in content {
                let dir_path = entry?.path();
                if dir_path.is_dir() {
                    let dir_path = dir_path.clone();
                    let dir_name = match dir_path.as_path().file_stem() {
                        Some(t) => t,
                        None => continue,
                    };
                    let dir_name = match dir_name.to_str() {
                        Some(t) => t.to_string(),
                        None => continue,
                    };
                    let mut cube = StaticMesh::new(dir_name, Matrix4::identity(), [0., 0., 0., 0.]);
                    let inner = fs::read_dir(dir_path.clone())?;
                    for file in inner {
                        let f = file?.path();
                        let file_stem = match f.file_stem() {
                            Some(t) => t.to_str().unwrap(),
                            None => continue,
                        };

                        let file_name = match f.file_name() {
                            Some(t) => t.to_str().unwrap(),
                            None => continue,
                        };

                        let lod = Lod::new(file_name.clone().to_string(), 0i8, 0.0, 4096.0);
                        cube.lods.insert((file_stem.split('_').collect::<Vec<&str>>().get(1).unwrap()).to_string().parse::<i8>().unwrap(), lod);
                    }
                    self.available_models.push(cube);
                }
            }
        }
        Ok(())
    }

    fn realtime_reload_objects() -> Receiver<(String, BufReader<File>)> {
        let (sender, receiver) = mpsc::channel();
        let thread = thread::spawn(move || {
            let (tx, rx) = channel();
            let mut watcher: RecommendedWatcher = Watcher::new_raw(tx).unwrap();
            let p = RESOURCES_PATH.to_string() + "/objects";

            let files_watched = watcher.watch(Path::new(&p), RecursiveMode::Recursive);
            loop {
                match rx.recv() {
                    Ok(notify::RawEvent { path: Some(path), op: Ok(op), cookie }) => {
                        println!("{:?} {:?} ({:?})", op, path, cookie);
                        let extension = path.extension();
                        let object_name = path.file_stem().unwrap().to_str().unwrap().to_string();
                        if extension == Some(OsStr::new("obj")) {
                            let mut file = File::open(path.clone());

                            match file {
                                Ok(file) => {
                                    let mut buff = BufReader::new(file);
                                    let mut raw_shader_data = Vec::new();
                                    buff.read_to_end(&mut raw_shader_data);
                                    sender.send((object_name, buff)).unwrap();
                                }
                                Err(e) => println!("Fail to open obj file")
                            }
                        }
                    }
                    Ok(event) => println!("broken event: {:?}", event),
                    Err(e) => println!("Fichier obj en erreur")
                }
            }
        });
        receiver
    }

    pub fn update_object_list(&mut self, objects_references: Vec<(String, Matrix4<f32>, i32, String)>) -> Result<(), Error> {
        let p = RESOURCES_PATH.to_string() + "/objects/";
        for object_ref in &objects_references {

            //TODO ajouter la suppression d'un modèle
            if self.available_models.iter().map(|x| x.clone().name).collect::<Vec<String>>().contains(&object_ref.3) {
                //TODO Gérer seulement la cas ou c'est une modification
                let available_models = self.available_models.iter().filter(|x| x.name == object_ref.3).map(|x| x.clone()).collect::<Vec<StaticMesh>>();
                let model_extracted = available_models.get(0).unwrap();
                let index = self.available_models.iter().position(|x| x.name == model_extracted.name).unwrap();
                self.available_models.remove(index);

                let mut mut_model_extracted = model_extracted.clone();
                let matrix = model_extracted.matrix;
                mut_model_extracted.matrix = object_ref.1;
                self.available_models.push(mut_model_extracted);
            } else {
                let temp_p = &p;
                let object_path = temp_p.to_string() + &object_ref.3;
                let path = Path::new(&object_path);
                if path.is_dir() {

//                    let parent_name = match object_name.4 {
//                        Some(t)=> t,
//                        None => "".to_string()
//                    };

                    let mut mesh = StaticMesh::new((&object_ref.3).to_string(), object_ref.1, [0., 0., 0., 0.]);
                    let inner = fs::read_dir(path.clone())?;
                    for file in inner {
                        let f = file?.path();
                        let file_stem = match f.file_stem() {
                            Some(t) => t.to_str().unwrap(),
                            None => continue,
                        };

                        let file_name = match f.file_name() {
                            Some(t) => t.to_str().unwrap(),
                            None => continue,
                        };

                        let lod = Lod::new(file_name.clone().to_string(), 0i8, 0.0, 4096.0);
                        if let Some(lod_index_string) = file_stem.split('_').collect::<Vec<&str>>().get(1){
                            let lod_index = (lod_index_string).to_string().parse::<i8>();

                            if let Ok(mesh_name) = lod_index {
                                mesh.lods.insert(mesh_name, lod);
                            }
                        }
                    }
                    self.available_models.push(mesh);
                    //ICI on a les bonnes données
                }
            }
        }

        Ok(())
    }

    pub fn get_objects_availables(&self) -> Vec<String> {
        self.available_models.clone().into_iter().map(|x| x.name).collect::<Vec<String>>()
    }

    pub fn get_buffers(display: &glium::Display, models: Vec<StaticMesh>) ->
    (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
        let mut vertice_array = Vec::new();
        for model in &models {
            for vertex in model.vertices.iter() {
                vertice_array.push(*vertex);
            }
        }

        let mut indice_array = Vec::new();
        let mut iterator: u16 = 0;
        for model in &models {
            let offset = model.indices.len() as u16;
            for index in model.clone().indices {
                indice_array.push(index + offset * iterator);
            }
            iterator = iterator + 1;
        }

        (glium::VertexBuffer::dynamic(display, &vertice_array).unwrap(),
         glium::index::IndexBuffer::new(display,
                                        glium::index::PrimitiveType::TriangleStrip,
                                        &indice_array).unwrap())
    }
}