use engine::vertex::{Vertex, Normal};
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub struct ObjImporter {}

impl ObjImporter {
    pub fn import(buff: BufReader<File>) -> (Vec<Vertex>, Vec<u16>) {
        println!("import file");
        let mut vertex_array: Vec<Vertex> = vec![];
        let mut normal_array: Vec<Normal> = vec![];
        let mut index_array: Vec<u16> = vec![];
        let mut normal_index_array: Vec<usize> = vec![];

        let mut faces = 0;


        for line in buff.lines() {
            let plop = line.unwrap();
            let mut array_whitespace = plop.split_whitespace();
            let prefix = array_whitespace.next().unwrap();


            if prefix == "v" {
                let tmp_vert = Vertex {
                    normal: (0.0, 0.0, 0.0),
                    position: (array_whitespace.next().unwrap().parse::<f32>().unwrap(), array_whitespace.next().unwrap().parse::<f32>().unwrap(), array_whitespace.next().unwrap().parse::<f32>().unwrap()),
                    tex_coords:[0.0f32,0.0]

                };

                let mut does_insert = true;
//                println!("Detection autres vert ");
//                for vert in &vertex_array {
//                    if vert == &tmp_vert {
//                        does_insert = false;
//                    }
//                }

//                println!("insertion autres vert");

                if does_insert {
                    vertex_array.push(tmp_vert);
                }
            }
//            println!("insertion autres vert");

            if prefix == "f" {
//                println!("1");
                let mut indice: (String, String, String);
                let mut normals: (u16, u16, u16);

                let v1 = array_whitespace.next().unwrap().split(r"/").collect::<Vec<&str>>();
                let v2 = array_whitespace.next().unwrap().split(r"/").collect::<Vec<&str>>();
                let v3 = array_whitespace.next().unwrap().split(r"/").collect::<Vec<&str>>();
//                println!("2");

                index_array.push(v1.get(0).unwrap().parse::<u16>().unwrap() - 1);
                normal_index_array.push(v1.get(2).unwrap().parse::<usize>().unwrap() - 1);
                index_array.push(v2.get(0).unwrap().parse::<u16>().unwrap() - 1);
                normal_index_array.push(v2.get(2).unwrap().parse::<usize>().unwrap() - 1);
                index_array.push(v3.get(0).unwrap().parse::<u16>().unwrap() - 1);
                normal_index_array.push(v3.get(2).unwrap().parse::<usize>().unwrap() - 1);
//                println!("3");

                faces += 1;
            }
//            println!("insertion normal");
            if prefix == "vn" {
                normal_array.push(Normal {
                    normal: (array_whitespace.next().unwrap().parse::<f32>().unwrap(), array_whitespace.next().unwrap().parse::<f32>().unwrap(), array_whitespace.next().unwrap().parse::<f32>().unwrap())
                });
            }
        }
        println!("import file end");
//        vertex_array.insert(0, Vertex {
//            position: (0.0, 0.0, 0.0)
//        });
//        normal_array.insert(0, Normal {
//            normal: (0.0, 0.0, 0.0)
//        });

//        println!("vertex count {}", vertex_array.len());
//        println!("index count {}", index_array.len());
//        println!("normals {:#?}", normal_array.len());
//        println!("faces {}", faces);
        println!("vertex {:#?}", vertex_array);
        (vertex_array, index_array)
    }

}