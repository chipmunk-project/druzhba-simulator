pub mod alu_parsing_utils;
pub mod alu_generation_utils;
pub mod rust_code_generator;
// Important: nightly must be enabled to work
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate lalrpop_util;
extern crate clap;

use std::env; 
use clap::{App, Arg};
lalrpop_mod!(pub alugrammar); // synthesized by LALRPOP


fn main() {

    let matches = App::new("dgen")
      .version("1.0")
      .about("Code generator for Druzhba")
      .arg(Arg::with_name("spec_name")
        .help("Name of input program")
        .index(1)
        .required(true)
      )
      .arg(Arg::with_name("stateful_alu")
        .help("Path to stateful ALU file")
        .index(2)
        .required(true)
      )
      .arg(Arg::with_name("stateless_alu")
        .help("Path to stateless ALU file")
        .index(3)
        .required(true)
      )
      .arg(Arg::with_name("pipeline_depth")
        .help("Depth of pipeline to simulate")
        .index(4)
        .required(true)
      )     
      .arg(Arg::with_name("pipeline_width")
        .help("Width of pipeline to simulate")
        .index(5)
        .required(true)
      )
      .arg(Arg::with_name("num_stateful_alus")
        .help("Number of stateful ALUs per stage")
        .index(6)
        .required(true)
      )     
      .arg(Arg::with_name("constant_vec")
        .help("Constant vector for Chipmunk")
        .index(7)
        .required(true)
      )
      .arg(Arg::with_name("machine_code")
        .help("Druzhba machine code (only required for optimzed code generation")
        .index(8)
        .required(false)
      )
      .arg(Arg::with_name("output_file")
        .short('o')
        .long("output")
        .help("Output generated file")
        .takes_value(true)
        .required(false)
      )
      .arg(Arg::with_name("opti")
        .short('O')
        .long("opti")
        .help("Optimization level: 0, 1, or 2")
        .takes_value(true)
        .required(false)
      )
      .get_matches();

    let spec_name : String = matches
          .value_of("spec_name")
          .unwrap()
          .to_string();
    let stateful_alu : String = matches
          .value_of("stateful_alu")
          .unwrap()
          .to_string();
    let stateless_alu : String = matches
          .value_of("stateless_alu")
          .unwrap()
          .to_string();

    let pipeline_depth : i32 = 
        match matches
            .value_of("pipeline_depth")
            .unwrap()
            .parse::<i32>() {
          Ok (t_pipeline_depth) => t_pipeline_depth,
          Err (_)               => panic!("Failure: Unbale to unwrap pipeline depth"),
        };
    let pipeline_width : i32 = 
        match matches
            .value_of("pipeline_width")
            .unwrap()
            .parse::<i32>() {
          Ok (t_pipeline_width) => t_pipeline_width,
          Err (_)               => panic!("Failure: Unbale to unwrap pipeline depth"),
        };
    let num_stateful_alus : i32 =  
        match matches
            .value_of("num_stateful_alus")
            .unwrap()
            .parse::<i32>() {
          Ok (t_pipeline_width) => t_pipeline_width,
          Err (_)               => panic!("Failure: Unbale to unwrap number of stateful ALUs"),
        };
    let constant_vec_string : String = matches
            .value_of("constant_vec")
            .unwrap()
            .to_string();
    let constant_vec : Vec <i32> = constant_vec_string
            .split(",")
            .map(|n| 
                 match n.trim().parse::<i32>() {
                   Ok (num) => num,
                   Err (_)  => panic!("Failrure: Unable to parse constant set"),
            })
            .collect();
    let file_path : String = matches
            .value_of("output_file")
            .unwrap_or("src/prog_to_run.rs")
            .trim()
            .to_string();

    let stateful_alu_split : Vec <String>= stateful_alu
            .split("/")
            .map (|s| s.to_string())
            .collect();
    let stateless_alu_split : Vec <String>= stateless_alu
            .split("/")
            .map (|s| s.to_string())
            .collect();


    let full_stateful_name = &stateful_alu_split[stateful_alu_split.len()-1].to_string();

    let stateful_name = &full_stateful_name[0..full_stateful_name.len()-4].to_string();
    let full_stateless_name = &stateless_alu_split[stateless_alu_split.len()-1].to_string();
    let stateless_name = &full_stateless_name[0..full_stateless_name.len()-4].to_string();
    let name : String = format!("{}_{}_{}_{}_{}", 
                                spec_name,
                                stateful_name,
                                stateless_name,
                                pipeline_depth,
                                pipeline_width);

    let opt_level = match matches
          .value_of("opti")
          .unwrap_or("0")
          .parse::<i32>() {
        Ok (num) => num,
        _        => panic!("Error: Invalid optimization level"),

    };
       
    if opt_level == 0 {
      alu_generation_utils::generate_alus (name, 
                                           stateful_alu, 
                                           stateless_alu, 
                                           pipeline_depth, 
                                           pipeline_width,
                                           num_stateful_alus,
                                           constant_vec,
                                           file_path,
                                           "".to_string(),
                                           0);
    }
    else {

      let machine_code_file : String = 
        match matches.value_of("machine_code") {
          Some (s) => s.to_string(),
          _        => panic!("Error: machine code file not provided"),
        };
      alu_generation_utils::generate_alus (name, 
                                           stateful_alu, 
                                           stateless_alu, 
                                           pipeline_depth, 
                                           pipeline_width,
                                           num_stateful_alus,
                                           constant_vec,
                                           file_path,
                                           machine_code_file,
                                           opt_level);
    }

    println!("dgen completed");
}

#[cfg(test)]
mod test_grammar;

