use crate::phv_container::PhvContainer;
use crate::phv::Phv;

/* Option being used for an output_value because their is no
  output_value until the mux actually executes */

#[derive(Clone)]
pub struct InputMux{
    pub input_phv: Phv<i32>,
    pub index : i32,
}

impl InputMux{

    pub fn new(&self, input: Phv<i32>, hole_index : i32) -> Self {
        InputMux {input_phv : input, index : hole_index}
    
    }
    pub fn output(&self) -> PhvContainer<i32> {

        if self.index >= self.input_phv.get_num_phv_containers() {
          self.input_phv [self.input_phv.get_num_phv_containers()-1].clone()
        }
        else {
          self.input_phv [self.index].clone()
        }
    }
}
