// 2D Fractal generator
// Mandelbrot set calculated according to http://www.hiddendimension.com/FractalMath/Divergent_Fractals_Main.html
// None of the code was copied, all custom made
// Created by Faras, 2017 & 2018
// Released under GPLv3 license

mod rustelbrot_2d;
mod rustelbrot_3dlayers;
mod rustelbrot_3dmesh;
mod rustelbrot_2dvid;

extern crate palette;
extern crate kiss3d;
extern crate nalgebra as na;
extern crate ncollide;

#[macro_use]
extern crate clap;

use clap::{App};

// use std::ops::Deref;

#[derive(Clone)]
enum GeneratorType {
    G2d,
    G2dVid,
    G3dMesh,
    G3dLayers
}


#[derive(Clone)]
pub struct Config<'a> {
    generator: GeneratorType,
    mesh: bool,
    balls: bool,
    frames: f64,
    dimentions: [f64;2],
    pixelsize: f64,
    boxstart: [f64;4],
    boxend: [f64;4],
    output_template:  &'a str,
}
static DEFAULTCONFIG:Config = Config {
    generator: GeneratorType::G2d,
    mesh: false,
    balls: false,
    frames: 10.0,
    dimentions: [500.0,500.0],
    pixelsize: 1.0,
    boxstart: [0.28,0.48,-0.50,-0.30],
    boxend: [0.4573671713,0.4573671717,-0.4068494815,-0.4068494811],
    output_template: "./generated/rustelbrot_f{:03}.png"
};

fn main() {
    let yml = load_yaml!("rustelbrot.yaml");
    let m = App::from_yaml(yml).get_matches();
    let mut config = DEFAULTCONFIG.clone();

    match m.subcommand() {
        ("3d",  Some(sub_m)) => {
            println!("3d");
            config.generator = GeneratorType::G3dMesh;

            if sub_m.is_present("mesh") {
                println!("mesh");
                config.mesh = true;
            }
            if sub_m.is_present("balls") {
                println!("balls");
                config.balls = true;
            }
            else if sub_m.is_present("layers") {
                println!("layers");
                config.generator = GeneratorType::G3dLayers;

                // mandelbrot3d::main();
            }
            else {
                println!("{}", sub_m.usage());
            }
        }, // clone was used
        ("vid",   Some(_)) => {
            println!("video");
            config.generator = GeneratorType::G2dVid;
        }, // push was used
        ("2d", Some(_)) => {
            println!("2d");
            config.generator = GeneratorType::G2d;
        }, // commit was used
        _  => {
            println!("{}", m.usage());

        } // Either no subcommand or one not tested for...
    }

    if m.is_present("pixelsize") {
        println!("config pixelsixe");
        config.pixelsize = match m.value_of("pixelsize") {
            Some(v) => v.parse::<f64>().unwrap(),
            None => DEFAULTCONFIG.pixelsize,
        }
    }
    if m.is_present("frames") {
        println!("config frames");
        config.frames = match m.value_of("frames") {
            Some(v) => v.parse::<f64>().unwrap(),
            None => DEFAULTCONFIG.frames,
        }
    }
    // if m.is_present("dimentions") {
    //     println!("config dimentions");
    //     config.dimentions = match m.value_of("dimentions") {
    //         Some(v) => v.parse::<String>().unwrap().split("x").filter_map(|x| x.parse::<f64>().ok()).collect::<Vec<f64>>().as_slice().deref(),
    //         None => DEFAULTCONFIG.dimentions,
    //     }
    // }
    // if m.is_present("frames") {
    //     println!("config frames");
    //     config.frames = match m.value_of("frames") {
    //         Some(v) => v.parse::<f64>().unwrap(),
    //         None => DEFAULTCONFIG.frames,
    //     }
    // }
    // if m.is_present("boxstart") {
    //     println!("config boxstart");
    //     config.boxstart = match m.value_of("boxstart") {
    //         Some(v) => v.parse::<f64>().unwrap(),
    //         None => DEFAULTCONFIG.boxstart,
    //     }
    // }
    // if m.is_present("boxend") {
    //     println!("config boxend");
    //     config.boxend = match m.value_of("boxend") {
    //         Some(v) => v.parse::<f64>().unwrap(),
    //         None => DEFAULTCONFIG.boxend,
    //     }
    // }
    // if m.is_present("output-template") {
    //     println!("config output-template");
    //     config.output_template = match m.value_of("output-template") {
    //         Some(v) => v.parse::<f64>().unwrap(),
    //         None => DEFAULTCONFIG.output_template,
    //     }
    // }
    //


    let generator = match config.generator {
        GeneratorType::G2d      => rustelbrot_2d::main,
        GeneratorType::G2dVid   => rustelbrot_2dvid::main,
        GeneratorType::G3dLayers=> rustelbrot_3dlayers::main,
        GeneratorType::G3dMesh  => rustelbrot_3dmesh::main,
    };

    generator(config);

}
