extern crate rand;

use crate::phv::Phv;
use crate::alu::ALU;
use crate::phv_container::PhvContainer;

pub struct PipelineStage {
   pub stateless_atoms : Vec<ALU>,
   pub stateful_atoms : Vec<ALU>,
   pub salu_configs : Vec<i32>,
   pub output_mux_globals : Vec<i32>,
   pub state_container : Vec<Vec<i32>>,
}

impl PipelineStage {
  pub fn new () -> Self {
    PipelineStage { stateless_atoms : Vec::new(),
                    stateful_atoms : Vec::new(), 
                    salu_configs : vec![0], 
                    output_mux_globals : vec![0],
                    state_container : Vec::new()}
  }
  pub fn with_atoms (stateless : Vec <ALU>, stateful : Vec<ALU>, t_salu_configs : Vec<i32>) -> Self{
    PipelineStage { stateless_atoms : stateless,
                    stateful_atoms : stateful, 
                    salu_configs : t_salu_configs,
                    output_mux_globals : vec![0],
                    state_container : Vec::new(),
    }
  }

  // Iterates through all atoms stored and calls their 
  // underlying function on the incoming Phv in 
  // random order. Pass the mutated phv containers to their respective muxes.
  pub fn tick(&mut self, 
              t_initial_phv : Phv <i32>,
              t_input_phv : Phv<i32>) -> (Phv<i32>,Phv<i32>){ 

      let mut input_phv : Phv <i32> = t_input_phv.clone();
      let mut initial_phv : Phv <i32> = t_initial_phv.clone();
      if input_phv.is_bubble() {
        (Phv::new(), Phv::new())
      }
      else{
        let mut output_phv : Phv<i32> = 
            Phv { bubble : false, 
                  packets: Vec::new(),
                  state : Vec::new()};


        let mut old_state : Vec <i32> = Vec::new();
        // List of new state variables for output mux
        let mut new_state : Vec <Vec <i32> > = Vec::new();
        // Need old state variables first to put them
        // into output muxes later
        let mut atom_count : usize = 0;

        for atom in self.stateful_atoms.iter_mut () {
          if self.salu_configs[atom_count] == 1 {

            if self.state_container.len() == 0 {
              self.state_container = input_phv.get_state();
              output_phv.set_state (input_phv.get_state());
            }
            // Update new phv state
            else {
              let mut t_input_state : Vec<Vec<i32>> = input_phv.get_state().clone();
              t_input_state[atom_count] = self.state_container[atom_count].clone();
              input_phv.set_state(t_input_state);

              let mut t_initial_state : Vec<Vec<i32>> = initial_phv.get_state();
              t_initial_state[atom_count] = self.state_container[atom_count].clone();
              initial_phv.set_state(t_initial_state);
            }

            atom.set_state_variables 
                (input_phv.get_state()[atom_count].clone());
          }
          atom.send_packets_to_input_muxes(input_phv.clone());
          let mut packet_fields : Vec<PhvContainer<i32>> = 
                atom.input_mux_output();
          let state_result = atom.run (&mut packet_fields);
          let mut old_state_result : Vec <i32> = state_result.0;

          let new_state_result : Vec <i32> = state_result.1;

          if self.output_mux_globals[atom_count] == 1 {
            old_state.append(&mut old_state_result);
          }
          else {
            old_state.append(&mut new_state_result.clone());
          }
          new_state.push (new_state_result);
          atom.reset_state_variables();
          atom_count+=1;
        }

        // Gets return values from the ALUs and inserts
        // them into output muxes along with old state vars
        for atom in self.stateless_atoms.iter_mut() {
        
          //PHV is passed to it's corresponding input mux, and
          //a single container is outputted. Container is put
          //into a vector and passed to atom
          atom.send_packets_to_input_muxes(input_phv.clone());
          let packet_fields : Vec<PhvContainer<i32>> = 
              atom.input_mux_output();
          //After being passed to atom, value is sent to an
          //output mux and put into a PHV

          let result : i32 =  atom.run(&packet_fields).0[0];
          // State variables and returned value from stateless ALU
          let mut output_mux_fields : Vec <i32> = old_state.clone();

          output_mux_fields.push (result);

          atom.send_packets_to_output_mux(&output_mux_fields);
          output_phv.add_container_to_phv(atom.output_mux.output());
        }
 
        // Update output_phv state variables
        let mut output_state : Vec <Vec <i32> > = Vec::new();
        for i in 0..self.salu_configs.len() {
          if self.salu_configs[i] == 1 {
            output_state.push (new_state[i].clone());
            // Write to state variables for next PHV
            self.state_container[i] = new_state[i].clone();
          }
          else {
            output_state.push (input_phv.get_state()[i].clone());
          }
        }

        output_phv.set_state (output_state);
        (initial_phv, output_phv)
      }
      
    }
    
  }

