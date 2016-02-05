use std::f64;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

extern crate csv;

struct Point {
    x: f64,
    y: f64,
}

fn dist(p1:&Point, p2:&Point) -> f64 {
    let x:f64 = p1.x-p2.x;
    let y:f64 = p1.y-p2.y;
    (x*x+y*y).sqrt()
}

fn hausdorff_dist(path1:&[Point], path2:&[Point]) -> f64 {
    // Calculate the Hausdorff distance of p1 from p2--symmetric
    let np:f64 = (path1.len()+path2.len()) as f64;
    let mut score:f64 = 0.0;
    for p1 in path1 {
        let mut min:f64 = f64::INFINITY;
        let mut d:f64;
        for p2 in path2 {
            d = dist(p1,p2);
            if d < min {min = d};
        }
        score = score+(min/np);
    }
    for p2 in path2 {
        let mut min:f64 = f64::INFINITY;
        let mut d:f64;
        for p1 in path1 {
            d = dist(p1,p2);
            if d<min {min = d};
        }
        score = score+(min/np);
    }
    score
}

fn import_path(file_path:&Path) -> Vec<Point> {
    let mut path: Vec<Point> = Vec::new();
    let mut reader = csv::Reader::from_file(file_path).unwrap();
    for row in reader.decode() {
        let (x,y): (f64, f64) = row.unwrap();
        path.push(Point {x:x,y:y});
    }
    path
}

fn get_path_files(folder:&str) -> Vec<String> {
    let v = vec!["unimplemented".to_string()];
    v
}

fn print_path(path:&[Point]) {
    for p in path {
        println!("  x:{}\ty:{}",p.x,p.y);
    }
}

fn main() {
    //points to test
    let fname = Path::new("path1.csv");
    let v1 = import_path(&fname); //vec![Point{x:1.0,y:1.0}];
    let v2 = vec![Point{x:1.0,y:1.0}];
    println!("length of vector 1: {}",  v1.len());
    print_path(&v1);
    let d = hausdorff_dist(&v1,&v2);
    println!("{}",d);
}
