pub mod alu_parsing_utils;
pub mod alu_generation_utils;
pub mod rust_code_generator;
// Important: nightly must be enabled to work
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate lalrpop_util;

use std::env; 
lalrpop_mod!(pub alugrammar); // synthesized by LALRPOP


fn main() {
    let args : Vec<String> = env::args().collect();
    // Make room for optional hole configs
    assert! (args.len() == 9 || 
             args.len() == 10 ||
             args.len() == 11);

    let spec_name : String = args[1].clone();
    let stateful_alu : String = 
        format!("{}", args[2].clone());
    let stateless_alu : String = 
        format!("{}", args[3].clone());
    let pipeline_depth : i32 = 
        match args[4].parse::<i32>() {
          Ok (t_pipeline_depth) => t_pipeline_depth,
          Err (_)               => panic!("Failure: Unbale to unwrap pipeline depth"),
        };
    let pipeline_width : i32 = 
        match args[5].parse::<i32>() {
          Ok (t_pipeline_width) => t_pipeline_width,
          Err (_)               => panic!("Failure: Unbale to unwrap pipeline depth"),
        };
    let num_stateful_alus : i32 =  
        match args[6].parse::<i32>() {
          Ok (t_pipeline_width) => t_pipeline_width,
          Err (_)               => panic!("Failure: Unbale to unwrap number of stateful ALUs"),
        };
    let constant_vec_string : String = args[7].clone();
    let constant_vec : Vec <i32> = constant_vec_string
                                   .split(",")
                                   .map(|n| 
                                        match n.trim().parse::<i32>() {
                                          Ok (num) => num,
                                          Err (_)  => panic!("Failrure: Unable to parse constant set"),
                                   })
                                    .collect();
    let file_path : String = args[8].clone();

    let stateful_alu_split : Vec <String>= stateful_alu.split("/")
                                                       .map (|s| s.to_string())
                                                       .collect();
    let stateless_alu_split : Vec <String>= stateless_alu.split("/")
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

    let opt_level : i32 = 
        match args.len() {
          // 11 args means hole configs and optimzation level supplied
          11  => match args[10].parse::<i32>() {
            Ok (t_opt_level) => t_opt_level,
            Err (_)          => panic!("Failure: Unable to unwrap optimization level"),
          },
          // 10 args means hole configs supplied but no optimization
          // level so set it to 1 by default
          10 => 1,
          _  => 0,

      };
    if opt_level == 0{
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

      let hole_cfg_file : String = args[9].clone();
      alu_generation_utils::generate_alus (name, 
                                           stateful_alu, 
                                           stateless_alu, 
                                           pipeline_depth, 
                                           pipeline_width,
                                           num_stateful_alus,
                                           constant_vec,
                                           file_path,
                                           hole_cfg_file,
                                           opt_level);
    }
    println!("dgen completed");
}

#[cfg(test)]
mod test_grammar;

