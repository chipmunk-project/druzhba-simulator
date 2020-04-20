#![feature(test)]
extern crate rand;
extern crate test;
extern crate druzhba;

use druzhba::pipeline::Pipeline;
use druzhba::phv::Phv;
use druzhba::phv_container::PhvContainer;

use rand::Rng;
use std::fs;
use std::collections::HashMap;
use test::Bencher;

pub mod blue_increase_unoptimized;
pub mod flowlets_unoptimized;
pub mod learn_filter_unoptimized;
pub mod rcp_unoptimized;

pub mod blue_increase_optimized_1;
pub mod flowlets_optimized_1;
pub mod learn_filter_optimized_1;
pub mod rcp_optimized_1;

pub mod blue_increase_optimized_2;
pub mod flowlets_optimized_2;
pub mod learn_filter_optimized_2;
pub mod rcp_optimized_2;

/* Test helper functions */
fn create_random_phvs (ticks : i32,
                   num_packets : i32,
                   pipeline_width : i32,
                   num_state_vars : i32,
                   num_stateful_alus : i32) -> Vec <Phv <i32> >
{
  let mut input_phvs : Vec <Phv <i32> > = Vec::new();
    // Initializes packet with all of the input fields
    // along with a random value
  for _i in 0..ticks {
    let mut packet : Phv<i32> = Phv::new();
    for _j in 0..num_packets{
        packet.add_container_to_phv(PhvContainer {
            field_value : rand::thread_rng().gen_range(0,100),
        }); 
    }
    for _j in num_packets..pipeline_width {
      packet.add_container_to_phv(PhvContainer {
          field_value : 0,
      }); 

    }
    let mut state_vec : Vec <Vec <i32>>  = Vec::new();
    for _i in 0..num_stateful_alus {
      let mut tmp_state_vec : Vec <i32> = Vec::new();
      for _j in 0..num_state_vars {
        tmp_state_vec.push (rand::thread_rng().gen_range(0,100));
      }
      state_vec.push (tmp_state_vec);
    }
    packet.set_state (state_vec);
    input_phvs.push (packet.clone());
  }
  input_phvs

}
// Initializes hole config HashMap and initializes pipeline.
// Runs input phvs through pipeline and returns the resulting
// output phvs
fn extract_hole_cfgs (hole_cfgs_file : String) -> HashMap <String, i32 > {
  let mut hole_cfgs_map : HashMap <String, i32> = HashMap::new();
  let hole_cfgs_file_contents : String = fs::read_to_string(hole_cfgs_file).expect ("Error: Hole configs file could not be found");
  let hole_cfgs_file_vec : Vec <String> = hole_cfgs_file_contents
                                          .split ("\n")
                                          .map (|s| s.to_string())
                                          .collect();

  for hole_var in hole_cfgs_file_vec {
      let hole_entry : Vec <&str> = hole_var
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

fn run_pipeline (input_phvs : Vec <Phv <i32> >,
                 pipeline : &mut Pipeline,
                 ticks : i32) -> (Vec<Phv<i32>>, Vec <Phv <i32>>) {
  let mut output_phvs : Vec <Phv <i32> > = Vec::new();
  let mut result_updated_input_phvs : Vec <Phv<i32> > = Vec::new();
  let mut result_output_phvs : Vec <Phv<i32> > = Vec::new();

  for t in 0..ticks {
    let updated_input_output_phvs : (Phv<i32>, Phv<i32>) = 
        pipeline.tick (input_phvs[t as usize].clone());

    if !updated_input_output_phvs.1.is_bubble() {
      result_output_phvs.push(updated_input_output_phvs.1);
      result_updated_input_phvs.push(updated_input_output_phvs.0);
    }
  }
  (result_updated_input_phvs, result_output_phvs)
}



#[bench]
fn bench_blue_increase_spec (b : &mut Bencher)
{
  let num_ticks : i32 = 10000;

  let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 2, 2, 2, 2);
  b.iter(|| {

    for i in 0..num_ticks {
      let mut expected : Phv <i32> = input_phvs[i as usize].clone();
      let mut state = expected.get_state();
      expected[1].field_value = expected[0].field_value - 1;
      if expected[1].field_value > state[1][0] {
        state[0][0] += 1;
        state[1][0] = expected[0].field_value;
      }
      expected.set_state(state);
                  
    }
  });
}

#[bench]
fn bench_flowlets_spec (b : &mut Bencher)
{
  let num_ticks : i32 = 10000;
  let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 3, 5, 1, 2);
  b.iter(|| {

    for i in 0..num_ticks {
      let mut expected : Phv <i32> = input_phvs[i as usize].clone();
      let mut state = expected.get_state();
      if expected[1].field_value - state[1][0] > 5 {
        state[0][0] = expected[0].field_value;
      }
      state[1][0] = expected[1].field_value;
      expected[2].field_value = state[0][0];
  
      expected.set_state(state);
       
    }
  });

}

#[bench]
fn bench_learn_filter_spec (b : &mut Bencher)
{
  let num_ticks : i32 = 10000;
  let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 1, 3, 1, 3);
  b.iter(|| {

    for i in 0..num_ticks {
      let mut expected : Phv <i32> = input_phvs[i as usize].clone();
      let mut state = expected.get_state();
      if state[2][0] != 0 && state[1][0] != 0 && state[0][0] != 0 {
        expected[0].field_value = 1;
      }
      state[2][0] = 1;
      state[1][0] = 1;
      state[0][0] = 1;
      expected.set_state(state);
    }
  });
}

#[bench]
fn bench_rcp_spec  (b : &mut Bencher){

  let num_ticks : i32 = 10000;
  let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 2, 3, 1, 3);
  b.iter(|| { 
    for i in 0..num_ticks {
      let mut expected : Phv <i32> = input_phvs[i as usize].clone();
      let mut state = expected.get_state();
      if expected[1].get_value() < 2 {
        state[1][0] = state[1][0] + expected[1].get_value();
        state[2][0] = state[2][0] + 1;
      }
      else {
        state[0][0] = expected[0].get_value() + state[0][0];
      }
      expected.set_state(state);
    }
  });
}


#[bench]
fn bench_blue_increase_dsim_unoptimized (b : &mut Bencher)
{
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 2, 2, 2, 2);
    let hole_cfg_file : String = String::from("hole_configurations/blue_increase_pair_stateless_alu_arith_4_2_hole_cfgs.txt");
    let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);

    let mut pipeline : Pipeline = 
      blue_increase_unoptimized::init_pipeline (hole_cfgs_map.clone());
    b.iter(||{

      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);
    });
}

#[bench]
fn bench_flowlets_dsim_unoptimized (b : &mut Bencher) {
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 3, 5, 1, 2);
    let hole_cfg_file : String = String::from("hole_configurations/flowlets_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt");
    let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);

    let mut pipeline : Pipeline = 
        flowlets_unoptimized::init_pipeline (hole_cfgs_map.clone());
    b.iter(||{
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);

    });
}

#[bench]
fn bench_learn_filter_dsim_unoptimized (b : &mut Bencher) {
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (20000, 1, 3, 1, 3);
    let hole_cfg_file : String = String::from("hole_configurations/learn_filter_equivalent_1_canonicalizer_equivalent_0_raw_stateless_alu_5_3_hole_cfgs.txt");
    let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);

    let mut pipeline : Pipeline = 
      learn_filter_unoptimized::init_pipeline (hole_cfgs_map.clone());
    b.iter(|| {
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);
    });
}

#[bench]
fn bench_rcp_dsim_unoptimized (b : &mut Bencher) {
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 1, 3, 1, 3);
    let hole_cfg_file : String = String::from("hole_configurations/rcp_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt");
    let hole_cfgs_map : HashMap <String, i32> = extract_hole_cfgs (hole_cfg_file);

    let mut pipeline : Pipeline = 
      rcp_unoptimized::init_pipeline (hole_cfgs_map.clone());

    b.iter(|| {
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);

    });
}


// Same tests after optimization (level 1)


#[bench]
fn bench_blue_increase_dsim_optimized_1 (b : &mut Bencher)
{
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 2, 2, 2, 2);

    let mut pipeline : Pipeline = 
      blue_increase_optimized_1::init_pipeline (HashMap::new());
    b.iter(||{
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);
    });
}

#[bench]
fn bench_flowlets_dsim_optimized_1(b : &mut Bencher) {
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 3, 5, 1, 2);

    let mut pipeline : Pipeline = 
      flowlets_optimized_1::init_pipeline (HashMap::new());
    b.iter(|| {
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);

    });
}

#[bench]
fn bench_learn_filter_dsim_optimized_1(b : &mut Bencher) {
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 1, 3, 1, 3);

    let mut pipeline : Pipeline = 
      learn_filter_optimized_1::init_pipeline (HashMap::new());
    b.iter(|| {
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);

    });
}

#[bench]
fn bench_rcp_optimized_1(b : &mut Bencher) {
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 1, 3, 1, 3);

    let mut pipeline : Pipeline = 
      rcp_optimized_1::init_pipeline (HashMap::new());
    b.iter(|| {

      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);

    });
}


// Same tests after optimization (level 2)


#[bench]
fn bench_blue_increase_dsim_optimized_2 (b : &mut Bencher)
{
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 2, 2, 2, 2);

    let mut pipeline : Pipeline = 
      blue_increase_optimized_2::init_pipeline (HashMap::new());
    b.iter(||{
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);


    });
}

#[bench]
fn bench_flowlets_dsim_optimized_2(b : &mut Bencher) {
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 3, 5, 1, 2);

    let mut pipeline : Pipeline = 
      flowlets_optimized_2::init_pipeline (HashMap::new());

    b.iter(|| {
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);

    });
}

#[bench]
fn bench_learn_filter_dsim_optimized_2(b : &mut Bencher) {
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 1, 3, 1, 3);

    let mut pipeline : Pipeline = 
      learn_filter_optimized_2::init_pipeline (HashMap::new());

    b.iter(|| {
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);

    });
}

#[bench]
fn bench_rcp_optimized_2(b : &mut Bencher) {
    let num_ticks : i32 = 10000;
    let input_phvs : Vec <Phv <i32> > = create_random_phvs (num_ticks, 1, 3, 1, 3);

    let mut pipeline : Pipeline = 
        rcp_optimized_2::init_pipeline (HashMap::new());

    b.iter(|| {
      let input_output_phvs : (Vec<Phv<i32>>, Vec<Phv<i32>>) = 
          run_pipeline (input_phvs.clone(),
                        &mut pipeline,
                        num_ticks);

    });
}


