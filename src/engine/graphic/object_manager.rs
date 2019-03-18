extern crate notify;
extern crate rand;
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
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use rand::Rng;

extern crate glium;

#[derive(Debug)]
pub struct ObjectManager {
    receiver: Receiver<(String, BufReader<File>)>,
    pub available_models: HashMap<String, RefCell<StaticMesh>>,
    pub models_loader_receiver: Vec<Receiver<(String, (Vec<Vertex>, Vec<u16>))>>,
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        ObjectManager {
            receiver: ObjectManager::realtime_reload_objects(),
            available_models: HashMap::new(),
            models_loader_receiver: vec![],
        }
    }

    pub fn update_lod(&mut self, camera_position: Vector3<f32>) {
        unimplemented!()
    }

    pub fn update_loaded_model_list(&mut self, camera_position: Vector4<f32>, model_to_load_names: Vec<String>) {
        println!("look for mesh");
//Trouver l'ensemble des mesh à afficher déjà présent dans le buffer

        for model in self.available_models.iter() {
            let model_clone = model.1.clone();
            let model_borrowed = model_clone.borrow();
            println!("Begin loading model {}", model_borrowed.name);
            let (sender, receiver) = mpsc::channel();
            self.models_loader_receiver.push(receiver);
            //TODO Gérer les niveau de details plus haut
            let object_name = &model_borrowed.name;

            let mut lod_level_to_load = -1;
            let mesh_name = {
                let distance = camera_position.distance(model_borrowed.matrix.row(2).clone());
                println!("distance {}", distance);
                if distance > 200.0 {
                    lod_level_to_load = 1i8;
                    if let Some(lod_name) = model_borrowed.lods.get(&1i8) {
                        lod_name.clone().mesh_name
                    }else{
                        "".to_string()
                    }
                } else {
                    lod_level_to_load = 0i8;
                    if let Some(lod_name) = model_borrowed.lods.get(&0i8) {
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
            if model_borrowed.actual_lod == lod_level_to_load {
                println!("No need to reload model");
                return;
            }

            let mut model_borrowed_mut = model.1.borrow_mut();
            model_borrowed_mut.actual_lod = lod_level_to_load;

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
//                            let obj_content = (vec![], vec![]);
                            println!("Obj loaded");
                            sender.send((object_name, obj_content)).unwrap();
                        }
                        Err(e) => println!("Fail to open obj file")
                    };
                };
            });
        }

    }

    pub fn load_models_into_buffer(&mut self) {
        for receiver in &self.models_loader_receiver {
            match receiver.try_recv() {
                Ok(t) => {

                    for available_model in self.available_models.values() {
                        if available_model.borrow().name == t.0.split('_').collect::<Vec<&str>>().get(0).unwrap().to_string() {

                            let mut model_mut = available_model.borrow_mut();
                            let vertex_index_clone = t.1.clone();
                            model_mut.vertices = vertex_index_clone.0;
                            model_mut.indices = vertex_index_clone.1;
                        }
                    }

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

                //TODO compute mesh (async?)
                unimplemented!()
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
                    let mut cube = StaticMesh::new("".to_string(),dir_name, Matrix4::identity(), [0., 0., 0., 0.]);
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
                    self.available_models.insert("".to_string(),RefCell::new(cube));
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
            if object_ref.3.clone() == "" {
                continue;
            }

            //TODO ajouter la suppression d'un modèle
            if self.available_models.contains_key(&object_ref.0.clone()) {
                //TODO Gérer seulement la cas ou c'est une modification

                if let Some(model) = self.available_models.get(&object_ref.0.clone()) {
                    {
                        let mut model_borrowed = model.borrow_mut();
                        model_borrowed.matrix = object_ref.1;
                    }
                    let m = model.borrow();
                }
            } else {
                let temp_p = &p;
                let object_path = temp_p.to_string() + &object_ref.3;
                let path = Path::new(&object_path);
                if path.is_dir() {

//                    let parent_name = match object_name.4 {
//                        Some(t)=> t,
//                        None => "".to_string()
//                    };

                    let mut rng = rand::thread_rng();
                    let mut mesh = StaticMesh::new(object_ref.0.clone()+&format!("{}", rng.gen::<u32>()), (&object_ref.3).to_string(), object_ref.1, [0., 0., 0., 0.]);
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
                        if let Some(lod_index_string) = file_stem.split('_').collect::<Vec<&str>>().get(1) {
                            let lod_index = (lod_index_string).to_string().parse::<i8>();

                            if let Ok(mesh_name) = lod_index {
                                mesh.lods.insert(mesh_name, lod);
                            }
                        }
                    }
                    self.available_models.insert(object_ref.0.clone(), RefCell::new(mesh));

                    for available_model in &self.available_models {
                        println!("{:?}", available_model.0);
                    }
                }
            }
        }

        Ok(())
    }

}