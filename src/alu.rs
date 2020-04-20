
use crate::phv_container::PhvContainer;
use crate::phv::Phv;
use crate::output_mux::OutputMux;
use crate::input_mux::InputMux;

//Use a PHV Container as an object containing all states - *just for now*
//return the previous states, unmodified or not

pub type StateVar = i32;
pub struct ALU{
    
    /*Packet containers from multiple muxs' are passed to an ALU, so should be a struct field and organized
    as a vector of containers compared to a PHV*/
    pub sequential_function :
        Box <dyn Fn (&mut Vec <StateVar>,
                     &Vec <PhvContainer<i32> >) 
                    -> (Vec <i32>, Vec <i32>) >,
    pub state_variables : Vec<i32>,
    pub input_muxes : Vec <InputMux> ,
    pub output_mux : OutputMux,
    pub is_stateful : bool,
}

impl ALU {
       
    pub fn new (function : Box <dyn Fn ( &mut Vec <i32>, 
                                &Vec<PhvContainer<i32>>) 
                                -> (Vec <i32>, Vec <i32>)>,
            t_state_variables : Vec <i32>,
            t_input_muxes: Vec <InputMux>,
            t_output_mux : OutputMux,
            t_is_stateful : bool) 
            -> ALU {

      ALU { sequential_function : function,
            state_variables: t_state_variables,
            input_muxes: t_input_muxes,
            output_mux : t_output_mux,
            is_stateful : t_is_stateful,
      }
    }

    // Receives mutable reference to Phv and calls underlying 
    // function, sequential_function using state_scalar and 
    // state_array. Mutates Phv in place with appropriate 
    // packet values. Once function is run, phv value should
    // be passed to the output mux. 
    pub fn run (&mut self, packet_fields: &Vec<PhvContainer<i32>>) -> (Vec <i32>, Vec <i32>) {

      (self.sequential_function) 
          (&mut self.state_variables,
           packet_fields
           )
    }

    //Helper functions to allow values to passed from and to muxs
    pub fn input_mux_output(&self) -> Vec<PhvContainer<i32> >{
        let mut packet_fields : Vec <PhvContainer <i32> > = 
            Vec::new();
        for mux in &self.input_muxes{
          packet_fields.push(mux.output());
        }
        packet_fields
    }
    pub fn send_packets_to_input_muxes(&mut self, values : Phv<i32>) {
        for mux in &mut self.input_muxes{
          mux.input_phv = values.clone();   
        }
    }
    pub fn send_packets_to_output_mux(&mut self, values: &Vec<i32>) {
      self.output_mux.swap_input_phv_containers (values);
    }  
    pub fn is_stateful (&self) -> bool {
        self.is_stateful
    }
    pub fn set_state_variables (&mut self, 
                                t_state_variables : Vec<i32>) {
      self.state_variables = t_state_variables;

    }
    pub fn get_state_varables (&self) -> Vec <i32> {
        self.state_variables.clone()
    }
    pub fn reset_state_variables (&mut self) {
      for i in 0..self.state_variables.len() {
        self.state_variables[i] = 0;
      }
    }
}


