use engine::vertex::{Vertex, Normal};
use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub struct ObjImporter{

}

impl ObjImporter {
    pub fn import(buff: BufReader<File>)->(Vec<Vertex>, Vec<u16>, Vec<Normal>){
        let mut vertex_array: Vec<Vertex> = vec![];
        let mut normal_array: Vec<Normal> = vec![];
        let mut index_array: Vec<u16> = vec![];
        let mut normal_index_array: Vec<usize> = vec![];


        for line in buff.lines() {
            let plop = line.unwrap();
            let mut array_whitespace = plop.split_whitespace();
            let prefix = array_whitespace.next().unwrap();


            if prefix == "v" {
                vertex_array.push(Vertex {
                    position:(array_whitespace.next().unwrap().parse::<f32>().unwrap(),array_whitespace.next().unwrap().parse::<f32>().unwrap(),array_whitespace.next().unwrap().parse::<f32>().unwrap())
//                    vertex: (array_whitespace.next().unwrap().to_string(),
//                             array_whitespace.next().unwrap().to_string(),
//                             array_whitespace.next().unwrap().to_string())
                });
            }

            if prefix == "f" {
                let mut indice: (String, String, String);
                let mut normals: (u16, u16, u16);

                let v1 = array_whitespace.next().unwrap().split(r"/").collect::<Vec<&str>>();
                let v2 = array_whitespace.next().unwrap().split(r"/").collect::<Vec<&str>>();
                let v3 = array_whitespace.next().unwrap().split(r"/").collect::<Vec<&str>>();


                normal_index_array.push(v1.get(2).unwrap().parse::<usize>().unwrap());
                normal_index_array.push(v2.get(2).unwrap().parse::<usize>().unwrap());
                normal_index_array.push(v3.get(2).unwrap().parse::<usize>().unwrap());

                index_array.push(v1.get(0).unwrap().parse::<u16>().unwrap());
                index_array.push(v2.get(0).unwrap().parse::<u16>().unwrap());
                index_array.push(v3.get(0).unwrap().parse::<u16>().unwrap());
            }

            if prefix == "vn" {
                normal_array.push(Normal {
                    normal: (array_whitespace.next().unwrap().parse::<f32>().unwrap(), array_whitespace.next().unwrap().parse::<f32>().unwrap(), array_whitespace.next().unwrap().parse::<f32>().unwrap())
                });
            }
        }
        (vertex_array, index_array, normal_array)
    }
}