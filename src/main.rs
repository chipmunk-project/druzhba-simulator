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
    let hole_cfgs_file_contents: String = fs::read_to_string(hole_cfgs_file.to_string())
          .expect (&format!("Error: Hole configs file {} could not be found",
                  hole_cfgs_file));
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
fn init_state_vector (num_stateful_alus: i32, 
                      num_state_values: i32) -> Vec<Vec<i32>> {
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

    let num_stateful_alus = prog_to_run::num_stateful_alus();
    let num_state_values = prog_to_run::num_state_variables();
    let mut phv: Phv<i32> = Phv::new();
    (0..num_phv_cons)
        .for_each( |_| {
         phv.add_container_to_phv(PhvContainer {
             field_value: rand::thread_rng().gen_range(0,100),
        }); 
     });
           
    (num_phv_cons..prog_to_run::pipeline_width())
        .for_each( |_| { 
            phv.add_container_to_phv (PhvContainer{
                field_value : 0,
            });
      }); 
    let state = init_state_vector (num_stateful_alus, 
         num_state_values);
    phv.set_state(state);
    phv
}

fn execute_pipeline (num_phv_cons: i32,
                     ticks: i32,
                     mut pipeline: Pipeline) {

    // For every tick create a new packet with the 
    // specified input fields set to random values from
    // 0 to 100. Send packet through pipeline and 
    // retrieve resulting packet.
    let mut input_phvs = Vec::new();
    let mut output_phvs = Vec::new();
    let mut state_string = "".to_string();
    // _t not used
    for t in 0..ticks {
        let phv = phv_generator(num_phv_cons);  
        if t == 0 {
            state_string = phv.get_state_string();
            println!("Initial state values: {}", state_string);
        }
        let updated_phv_pair = pipeline.tick (phv);
        let updated_input_phv = updated_phv_pair.0;
        let output_phv = updated_phv_pair.1;
        if !output_phv.is_bubble() {
          input_phvs.push((t+1-prog_to_run::pipeline_depth(), 
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
          .about("Hardware switch simulator for compiler testing")
          .arg(Arg::new("num_phv_cons")
               .short('g')
               .long("gen")
               .about("Number of PHV containers to be initialized by traffic generator")
               .takes_value(true)
               .required(false)
          )
          .arg(Arg::new("ticks")
               .short('t')
               .long("ticks")
               .about("Number of ticks to execute for. A PHV enters the pipeline at every tick.")
               .takes_value(true)
               .required(false)
          )
          .arg(Arg::new("input_file")
               .short('i')
               .long("input")
               .about("Path to file containing machine code pairs.")
               .takes_value(true)
               .required(false)
          ).get_matches();
    
      let num_phv_containers = 
          match matches.value_of("num_phv_cons") {
              Some (t_num_phv_containers) => 
                  match t_num_phv_containers.parse::<i32> () {
                      Ok(value) => value, 
                      _ => panic!("Error: Invalid num_phv_containers provided")
              },
           _ => prog_to_run::pipeline_width(),
      };
      let ticks = 
          match matches.value_of("ticks") {
              Some (t_ticks) => match t_ticks.parse::<i32> () {
                  Ok(value) => value,
                  _  => panic!("Failure: Unable to unwrap ticks"),
              }
              _ => 100,
          };
      let file = matches.value_of("input_file")
          .unwrap_or("")
          .trim()
          .to_string();
    
      assert! (ticks >= 1);
      assert! (prog_to_run::num_stateful_alus()>=1);
      println!("File: {}", file);
      let pipeline : Pipeline = 
          match file.as_str() {
             "" => prog_to_run::init_pipeline(HashMap::new()),
             _  => prog_to_run::init_pipeline(
                     extract_hole_cfgs(file.to_string())
              ),
          };
      println!("Executing pipeline");
      execute_pipeline (num_phv_containers, ticks, pipeline);
}
#[cfg(test)]
mod test_druzhba;
mod test_with_chipmunk;

