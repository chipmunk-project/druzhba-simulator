extern crate rand;
extern crate druzhba;
extern crate clap;

mod prog_to_run;
mod tests;

use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use clap::{App, Arg};

// Opens hole configs file of hole variable assignments
// and initializes a HashMap from hole vaiables to
// i32s.
fn extract_hole_cfgs (hole_cfgs_file: String) -> HashMap <String, i32> {

    println!("Extracting machine code pairs");
    let mut hole_cfgs_map: HashMap <String, i32> = HashMap::new();
    let hole_cfgs_file_contents: String = 
        match fs::read_to_string(hole_cfgs_file.to_string()) {
            Ok(f) => f,
            _ => {
                println!("Error: Hole configs file {} could not be found", hole_cfgs_file);
                panic!("Error: Hole configs file {} could not be found", hole_cfgs_file)
            },
        };
    let hole_cfgs_file_vec: Vec <String> = hole_cfgs_file_contents
        .split ("\n")
        .map (|s| s.to_string())
        .collect();
    for hole_var in hole_cfgs_file_vec {
        let hole_entry: Vec <&str> = hole_var
            .split("=")
            .map(|s| s.trim())
            .collect();
        if hole_entry.len() < 2 {
          continue;
        }
        hole_cfgs_map.insert (hole_entry[0].to_string(), 
                              hole_entry[1].to_string().parse::<i32>()
                                  .expect ("Error: hole value set to non-integer value"));
    }
    hole_cfgs_map
}
fn create_state_vector () -> Vec<Vec<i32>> {
    let num_stateful_alus = prog_to_run::num_stateful_alus();
    let num_state_values = prog_to_run::num_state_variables();

    let mut state = Vec::new();
    for _ in 0..num_stateful_alus{
        let mut tmp_state_vec = Vec::new();
        for _ in 0..num_state_values {
            tmp_state_vec.push(rand::thread_rng().gen_range(0,100));
        }
        state.push (tmp_state_vec);
    }
    state
}
// Generate PHV and initialize state
fn phv_generator (num_phv_cons: i32) -> Phv <i32>{

    let mut phv: Phv<i32> = Phv::new();
    (0..num_phv_cons)
        .for_each( |_| {
         phv.add_container_to_phv(PhvContainer {
             field_value: rand::thread_rng().gen_range(0,100),
        }); 
     });
           
    (num_phv_cons..prog_to_run::pipeline_width())
        .for_each( |_| { 
            phv.add_container_to_phv (PhvContainer {
                field_value : 0,
            });
        }); 
    let state = create_state_vector();
    phv.set_state(state);
    phv
        
}

fn create_phv (num_phv_cons: i32, phv_values: &Vec<i32>) -> Phv<i32> {

    if phv_values.len() == 0 {
        phv_generator(num_phv_cons)
    }
    else {

        let mut phv: Phv<i32> = Phv::new();
        if phv_values.len() >= prog_to_run::pipeline_width() as usize {
            for i in 0..prog_to_run::pipeline_width() {
                phv.add_container_to_phv(PhvContainer {
                    field_value: phv_values[i as usize],
                });
            }
        }
        else {
            for p in phv_values.iter() {
                phv.add_container_to_phv(PhvContainer {
                    field_value: *p,
                });
            }
            if num_phv_cons as usize > phv_values.len() {
                (phv_values.len()..num_phv_cons as usize)
                    .for_each(|_| {
                        phv.add_container_to_phv(PhvContainer {
                            field_value: rand::thread_rng().gen_range(0,100),
                        }); 
                    }); 
            }
            if prog_to_run::pipeline_width() > phv.get_num_phv_containers() {
                (phv.get_num_phv_containers()..prog_to_run::pipeline_width())
                    .for_each( |_| { 
                        phv.add_container_to_phv(PhvContainer {
                            field_value: 0,
                        });
                    }); 
            }
        }
        let state = create_state_vector();
        phv.set_state(state);
        phv
    }
}
fn strip_curly_braces_from_str <'a> (s: &'a str) -> &'a str {

    let begin_idx = s.find('{').unwrap();
    let end_idx = s.rfind('}').unwrap();
    &s[begin_idx + 1..end_idx]
}

fn ret_vec_str_elements(s: &str) -> Vec<i32> {

    let vec: Vec<i32> = s.split(",").map(|n|  {
        match n.trim().parse::<i32> () {
            Ok(val) => val,
             _  => {
                println!("Failure: state vector elements invalid");
                panic!("Failure: state vector elements invalid")
            }
        }
    }).collect();

    let state_len = prog_to_run::num_state_variables() as usize;
    match vec.len() == state_len {
        true => vec,
        _ => {

            println!("Invalid state vector length");
            panic!("Failure: state vector has incorrect size")

        }
    }
}

fn convert_init_state_vector_str (init_state_vector_str: &str) -> Vec<Vec<i32>>{
    if init_state_vector_str.contains("{}") 
    || !init_state_vector_str.contains("{") 
    || !init_state_vector_str.contains("}") {
        println!("Improper state format provided. Initializing random values");
        return Vec::new();
    }
    let stripped_vec_str = strip_curly_braces_from_str(init_state_vector_str);
    let group_len = prog_to_run::num_stateful_alus() as usize;

    let mut slice = stripped_vec_str;
    let mut state_vec = Vec::new();
    while slice.contains("{") && slice.contains("}"){
        let vec = ret_vec_str_elements(&slice
            [slice.find("{").unwrap() + 1..slice.find("}").unwrap()]);
        state_vec.push(vec);
        slice = &slice[slice.find("}").unwrap() + 1..];
    }
    if state_vec.len() != group_len {
        println!("Invalid state vector length");
        panic!("Failure: state vector sizes are invalid");
    }
    else {
        state_vec
    }
}

fn convert_phv_str (phv_str: &str) -> Vec<i32> {
    if phv_str == "" {
        return Vec::new();
    }
    phv_str.split(",")
        .map(|n| match n.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_)  => {
                println!("Incorrect value provided for PHV container");
                panic!("Failure: unabel to parse given PHV container values")
            },
    })
        .collect()
}
fn execute_pipeline (num_phv_cons: i32,
                     phv_values: Vec<i32>,
                     init_state_vec: Vec<Vec<i32>>,
                     ticks: i32,
                     mut pipeline: Pipeline) {

    // For every tick create a new packet with the 
    // specified input fields set to random values from
    // 0 to 100. Send packet through pipeline and 
    // retrieve resulting packet.
    let mut input_phvs = Vec::new();
    let mut output_phvs = Vec::new();
    let mut state_string = "".to_string();
    println!();
    println!("phv values: {:?}", phv_values);
    // _t not used
    for t in 0..ticks {
        let mut phv = create_phv(num_phv_cons, &phv_values);  
        if t == 0 {
            if init_state_vec.len() > 0 {
                phv.set_state(init_state_vec.clone());
            }
            state_string = phv.get_state_string();
            println!("Initial state values: {}", state_string);
        }
        let updated_phv_pair = pipeline.tick(phv);
        let updated_input_phv = updated_phv_pair.0;
        let output_phv = updated_phv_pair.1;
        if !output_phv.is_bubble() {
          input_phvs.push((t + 1 - prog_to_run::pipeline_depth(), 
            updated_input_phv.clone()));
          output_phvs.push((t, output_phv.clone()));
        }
    }
    for i in 0..output_phvs.len(){
        println!("Beginning of tick {}: Input PHV prior to pipeline entry:\n{}", 
            input_phvs[i].0, input_phvs[i].1);
        println!("End of tick {}: Output PHV following pipeline execution:\n{}\n{}\n\n", 
            output_phvs[i].0, output_phvs[i].1, 
            output_phvs[i].1.get_state_string());
    }
    println!("\n(Initial state variables were:\n{})", 
        state_string);
}
  
    #[warn(unused_imports)]
    fn main() {

      let matches = App::new("dsim")
          .version("1.0")
          .about("Hardware switch simulator")
          .arg(Arg::with_name("num_phv_cons")
               .short("g")
               .long("gen")
               .help("Number of PHV containers to be initialized by traffic generator")
               .takes_value(true)
               .required(false)
          )
          .arg(Arg::with_name("ticks")
               .short("t")
               .long("ticks")
               .help("Number of ticks to execute for. A PHV enters the pipeline at every tick.")
               .takes_value(true)
               .required(false)
          )
          .arg(Arg::with_name("input_file")
               .short("i")
               .long("input")
               .help("Path to file containing machine code pairs.")
               .takes_value(true)
               .required(false)
          )
          .arg(Arg::with_name("phv_initialization")
               .short("p")
               .long("phv")
               .help("Initial PHV values in form \"x_1, x_2, ... \"")
               .takes_value(true)
               .required(false)
          )
          .arg(Arg::with_name("init_state_vector")
               .short("s")
               .long("state")
               .help("pipeline state variable values (provided in the form: \"{{state_group_0_state_0, state_group_0_state_1, ...}, {sta    te_group_1_state_0, state_group_1_state_1, ...}, ...}\"")
               .takes_value(true)
               .required(false)
          ).get_matches();
    
      let num_phv_containers = 
          match matches.value_of("num_phv_cons") {
              Some (t_num_phv_containers) => 
                  match t_num_phv_containers.parse::<i32> () {
                      Ok(value) => value, 
                      _ => {
                          println!("Error: Invalid num_phv_containers provided");
                          panic!("Error: Invalid num_phv_containers provided")
                      },
              },
           _ => prog_to_run::pipeline_width(),
      };
      let ticks = 
          match matches.value_of("ticks") {
              Some (t_ticks) => match t_ticks.parse::<i32> () {
                  Ok(value) => value,
                  _  => {
                      println!("Failure: Unable to unwrap ticks");
                      panic!("Failure: Unable to unwrap ticks")
                  },
              }
              _ => 100,
          };

      let file = matches.value_of("input_file")
          .unwrap_or("")
          .trim()
          .to_string();

      let init_state_vector_str = 
          match matches.value_of("init_state_vector") {
              Some(s) => s,
              _       => "{}",
          };
      let phv_values = 
          match matches.value_of("phv_initialization") {
              Some(s) => convert_phv_str(s),
              _       => Vec::new(),
          };
      assert! (ticks >= 1);
      assert! (prog_to_run::num_stateful_alus() >= 1);
      println!("File: {}", file);
      let pipeline = match file.as_str() {
           "" => prog_to_run::init_pipeline(HashMap::new()),
           _  => prog_to_run::init_pipeline(
                   extract_hole_cfgs(file.to_string())
            ),
      };
      let init_state_vector = match init_state_vector_str {
          "{}" => Vec::new(),
          _ => convert_init_state_vector_str(init_state_vector_str),
      };
      println!("Executing pipeline");
      execute_pipeline (num_phv_containers,
          phv_values,
          init_state_vector, 
          ticks, 
          pipeline);
}
#[cfg(test)]
mod test_druzhba;
mod test_with_chipmunk;

