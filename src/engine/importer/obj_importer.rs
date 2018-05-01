use engine::vertex::{Vertex, Normal};
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub struct ObjImporter {}

impl ObjImporter {
    pub fn import(buff: BufReader<File>) -> (Vec<Vertex>, Vec<u16>) {
        println!("import file");
        let mut vertex_array: Vec<(f32, f32, f32)> = vec![];
        let mut normal_array: Vec<(f32, f32, f32)> = vec![];
        let mut uv_array: Vec<(f32, f32)> = vec![];
        let mut index_array: Vec<(u16, u16, u16)> = vec![];
        let mut normal_index_array: Vec<usize> = vec![];

        let mut faces = 0;


        for line in buff.lines() {
            let plop = line.unwrap();
            let mut array_whitespace = plop.split_whitespace();
            let prefix = array_whitespace.next().unwrap();


            if prefix == "v" {
                let tmp_vert_position = (array_whitespace.next().unwrap().parse::<f32>().unwrap(),
                                         array_whitespace.next().unwrap().parse::<f32>().unwrap(),
                                         array_whitespace.next().unwrap().parse::<f32>().unwrap());

                vertex_array.push(tmp_vert_position);
            }
            if prefix == "vn" {
                normal_array.push((array_whitespace.next().unwrap().parse::<f32>().unwrap(),
                                   array_whitespace.next().unwrap().parse::<f32>().unwrap(),
                                   array_whitespace.next().unwrap().parse::<f32>().unwrap()));
            }
            if prefix == "vt" {
                uv_array.push((array_whitespace.next().unwrap().parse::<f32>().unwrap(),
                               array_whitespace.next().unwrap().parse::<f32>().unwrap()))
            }

            if prefix == "f" {
                let mut indice: (String, String, String);
                let mut normals: (u16, u16, u16);
                let mut uv: (u16,u16);

                let v1 = array_whitespace.next().unwrap().split(r"/").collect::<Vec<&str>>();
                let v2 = array_whitespace.next().unwrap().split(r"/").collect::<Vec<&str>>();
                let v3 = array_whitespace.next().unwrap().split(r"/").collect::<Vec<&str>>();

                index_array.push((v1.get(0).unwrap().parse::<u16>().unwrap() - 1,
                                 v1.get(1).unwrap().parse::<u16>().unwrap() - 1,
                                  v1.get(2).unwrap().parse::<u16>().unwrap() - 1));
                index_array.push((v2.get(0).unwrap().parse::<u16>().unwrap() - 1,
                                  v2.get(1).unwrap().parse::<u16>().unwrap() - 1,
                                  v2.get(2).unwrap().parse::<u16>().unwrap() - 1));
                index_array.push((v3.get(0).unwrap().parse::<u16>().unwrap() - 1,
                                  v3.get(1).unwrap().parse::<u16>().unwrap() - 1,
                                  v3.get(2).unwrap().parse::<u16>().unwrap() - 1));

                faces += 1;
            }
        }

        let mut vert_array_result = vec![];

        for index in &index_array {
            vert_array_result.push(Vertex {
                position: *vertex_array.get(index.0 as usize).unwrap(),
                normal: *normal_array.get(index.2 as usize).unwrap(),
                tex_coords: *uv_array.get(index.1 as usize).unwrap(),
            });
        }

        let plop = index_array.iter().map(|x| x.0).collect::<Vec<u16>>();


        (vert_array_result, plop)
    }
}