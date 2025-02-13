//encoding=utf-8
// 3d Fractal generator
// Mandelbrot set calculated according to http://www.hiddendimension.com/FractalMath/Divergent_Fractals_Main.html
// None of the code was copied, all custom made
// Created by Faras, 2017 & 2018
// Released under GPLv3 license

// extern crate palette;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate ncollide3d;

use crate::Config;

use std::f64;
use std::f64::consts::E;
use std::time::Instant;
// use std::path::Path;

use std::rc::Rc;
use std::cell::RefCell;

use kiss3d::resource::{Material};
use kiss3d::builtin::NormalsMaterial;

// use self::palette::{Hsv, RgbHue,Gradient};

use self::na::{Vector3, Point3};
use self::kiss3d::window::Window;
use self::kiss3d::light::Light;
// use kiss3d::resource::Texture;

// use ncollide::ncollide_procedural::TriMesh;
use self::ncollide3d::procedural::quad_with_vertices;
// use self::ncollide::ncollide_procedural::quad;
// use ncollide::math::Point as P;

// this function tries to determine at which speed does the recursive function blow up
fn unbound_speed(x: f64,y: f64) -> f64 {
    let mut z0 = 0.0;
    let mut z1 = 0.0;
    let mut s = 0.0;
    let iterations_per_pixel = 80;
    let mut i = 0;

    'lo: loop {
        let (z2,z3) = recursive(z0,z1,x,y);
        if z2 == z0 || z2.is_nan() || z2 > 4.0 {
            break 'lo;
        }
        z0 = z2;
        z1 = z3;

        let p = E**&((&z2+&z3).abs()*-1.0);
        // println!("u z2 {} z3 {} ",z2,z3);
        s = s + p;

        i = i+1;
        if i > iterations_per_pixel {
            break 'lo;
        }
    }

    return s
}

// the recursive function is the one needed for the mandelbrot set, it operates on complex numbers (actually, two tuples)
fn recursive(zr:f64,zi:f64,cr:f64,ci:f64) -> (f64,f64) {
    // formula: zn+1 = z2n + c

    // Check for numbers too large or small to handle
    // if (z0 > 0.0 && z0 < 1e32) || (z0 < 0.0 && z0 > -1e32) || (z0 == 0.0) {
    let z2r = zr * zr - zi*zi;
    let z2i = zr * zi + zr * zi;
    // }
    // else {
    //     return (z0,z1);
    // }

    // addition: (a + bi) + (c + di) = (a + c) + (b + d)i
    let z2cr = z2r+cr;
    let z2ci = z2i+ci;

    // println!("{:?}",z2cr);

    return (z2cr,z2ci)
}

fn map_range_log(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 +
        (s - from_range.0) *
        (to_range.1 - to_range.0) /
        (from_range.1 - from_range.0) *
        (
            &2.0 -
            (s - from_range.0) /
            (from_range.1 - from_range.0)
        )
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 +
        (s - from_range.0) *
        (to_range.1 - to_range.0) /
        (from_range.1 - from_range.0)
}

pub fn main(config:Config) {
    let start = Instant::now();

    let mut window = Window::new("Rustelbrot3d");

    window.set_light(Light::StickToCamera);

    let boxi = config.boxstart;

    let precissionx:f64 = (&boxi[1]-&boxi[0])/&(config.dimentions[0]) * &config.pixelsize;
    let precissiony:f64 = (&boxi[3]-&boxi[2])/&(config.dimentions[1]) * &config.pixelsize;

    // let hue_shift = 0.0;

    // let gradient:Gradient<Hsv> = Gradient::new(vec![
        //  Hsv::new(RgbHue::from(-90.0+hue_shift), 1.0, 1.0)
        // ,Hsv::new(RgbHue::from(-80.0+hue_shift), 0.4, 0.4)
        // ,Hsv::new(RgbHue::from(-70.0+hue_shift), 0.5, 0.5)
        // ,Hsv::new(RgbHue::from(-61.0+hue_shift), 0.6, 0.6)
        // ,Hsv::new(RgbHue::from(-50.0+hue_shift), 0.7, 0.7)
        // ,Hsv::new(RgbHue::from(-20.0+hue_shift), 0.8, 0.8)
        // ,Hsv::new(RgbHue::from( -0.0+hue_shift), 1.0, 0.7)
        // ,Hsv::new(RgbHue::from( 10.0+hue_shift), 0.5, 0.7)
        // ,Hsv::new(RgbHue::from( 50.0+hue_shift), 0.2, 0.9)
        // ,Hsv::new(RgbHue::from( 61.0+hue_shift), 0.1, 1.0)
        // ]
    // );

//    println!("hs{:?}", hue_shift);
    // //
    // println!("py{:?}", precissiony);
    // println!("px{:?}", precissionx);

    let mut x:f64 = boxi[0];

    let mut vertices = vec![];
    // let p2 = Point3::new(1.0,1.0,1.0);
    // let v = Vector3::new(1.0,1.0,1.0);

    while x <= boxi[1] {
        // println!("{}",x);
        let mut y:f64 = boxi[2];
        while y <= boxi[3] {

            let realx = map_range((boxi[0],boxi[1]),(0.0,1.0),x);
            let realy = map_range((boxi[2],boxi[3]),(0.0,1.0),y);

            let z = unbound_speed(x,y);

            let mut z1 = map_range((-1e2 as f64,-1e1 as f64),(0.1,0.2),z);

            //Limit max depth
            if z1 < 0.0 {
                z1 = map_range_log((-1e308 as f64,-1e2 as f64),(0.0,0.1),z);
                if z1 < 0.0 {
                    println!("a{}",z1);
                    z1 = -0.0;
                }

            }
            // if z1 > 0.6 {
            //     println!("m{}",z1);
            //     z1 = 0.6
            // }
            let z2 = z1 + (x*30.0).sin()/5.0 - (y*30.0).cos()/5.0;
            // println!("{} cos{}",z1,z2);


            let p = Point3::new(realx as f32,realy as f32,(z2) as f32);
            // print!("{}",p);
            // let p = Point3::new(realx as f32,realy as f32,z1 as f32);
            // let pmesh = p;//Point3::new(0.0,0.0,1.0);



            vertices.push(p);

            y+=precissiony;
        }
        x+=precissionx;
    }
    // println!("vertices:{:?}",vertices );

    let mut m;

    let quad = quad_with_vertices(&vec![],(config.dimentions[0]/(config.pixelsize)) as usize,(config.dimentions[1]/config.pixelsize) as usize);
    // let mut quad = quad(1.0,1.0,(config.dimentions[0]/(config.pixelsize)) as usize,(config.dimentions[1]/config.pixelsize) as usize);
    // println!("quad:{:?}",quad );
    // m = window.add_trimesh(quad.clone(),Vector3::new(1.0,1.0,1.0));

    // https://github.com/sebcrozet/kiss3d/blob/master/examples/custom_material.rs
    let material   = Rc::new(RefCell::new(Box::new(NormalsMaterial::new()) as Box<Material + 'static>));

    // let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    // m.set_texture_from_file(&Path::new("/var/www/matherial/rustelbrot/generated/rustelbrot_f050.png"),&"textura");
    // let mut current_frame = config.frames;
    let mut quad = quad.clone();
    quad.coords = vertices.clone();
    m = window.add_trimesh(quad,Vector3::new(1.0,1.0,1.0));
    m.set_material(material.clone());
    m.recompute_normals();
    //
    let p = Point3::new(0.0,0.0,0.0);
    let p2 = Point3::new(0.0,0.0,1.0);
    let v = Vector3::new(0.9,0.9,0.9);
    m.reorient(&p,&p2,&v);

    while window.render() {

    }

    let duration = start.elapsed().as_secs() as f64 + start.elapsed().subsec_nanos() as f64  * 1e-9;

    println!("Init time {} seconds until first render.",duration );



}
