use crate::pipeline_stage::PipelineStage;
use crate::phv::Phv;
use std::mem;
use std::collections::HashMap;
use std::fmt;

 /*
    Banzai uses write and read latches to make sure that the output values of
    one stage don't become the input values of the next stage in a single tick (by writing an output into the read field). Im just using
    two hashmaps as a work-around.
*/

pub struct Pipeline {
    pipeline_stages : Vec<PipelineStage>,
    
    //Format : key = stage_number, value = PHV
    old_phvs: HashMap<usize, Phv<i32>>,
    new_phvs: HashMap<usize, Phv<i32>>,
    old_initial_phvs : HashMap<usize, Phv<i32>>,
    new_initial_phvs : HashMap<usize, Phv<i32>>
}

impl Pipeline {
    pub fn new () -> Self {
      let stages: Vec <PipelineStage> = Vec::new();
      Pipeline { pipeline_stages : stages, 
                 old_phvs: HashMap::new(), 
                 new_phvs : HashMap::new(),
                 old_initial_phvs : HashMap::new(),
                 new_initial_phvs : HashMap::new()}
    }
  
    pub fn with_pipeline_stages (t_pipeline_stages: Vec <PipelineStage>) -> Self {
        let mut empty_phvs = HashMap::new();
        for i in 0..t_pipeline_stages.len(){
          empty_phvs.insert(i as usize, Phv::new());
        }
        Pipeline { pipeline_stages : t_pipeline_stages, 
                   old_phvs: empty_phvs.clone(), 
                   new_phvs: empty_phvs.clone(),
                   old_initial_phvs : empty_phvs.clone(),
                   new_initial_phvs : empty_phvs.clone()}
    }
  
    pub fn len (&self) -> usize {
        self.pipeline_stages.len()
    }
    // Pumps a phv into the pipeline and runs the ALUs in the
    // first stage on them. Phvs later in the pipeline are
    // funneled to the next sequential stage. 
    //
    // initial_phv is not actually written to. It's
    // just the initial values of a PHV before it entered
    // the pipeline. Returned is a pair where the first 
    // element is a copy of the PHV when it first entered
    // the pipeline and the second element is the PHV after
    // it's values have been altered. This second element
    // also contains the resulting state values
    pub fn tick (&mut self, 
                 t_phv: Phv<i32>) -> (Phv<i32>, Phv<i32>) {
        if self.pipeline_stages.len() == 1 {
            (t_phv.clone(), self.pipeline_stages[0].tick(t_phv)) 
        }
        else {
            let first_result_phv = 
                self.pipeline_stages[0].tick(t_phv.clone());
      
            self.new_phvs.insert(0, first_result_phv);
            self.new_initial_phvs.insert(0, t_phv.clone());
            for x in 1..self.pipeline_stages.len() - 1 {
                let result_phv = self.pipeline_stages[x].
                    tick(self.old_phvs[&(x-1)].clone());
                self.new_phvs.insert(x, result_phv);
                self.new_initial_phvs.insert(x, 
                    self.old_initial_phvs[&(x-1)].clone());
            }
            let length = self.pipeline_stages.len();
            let last_phv = self.pipeline_stages[length - 1]
                 .tick(self.old_phvs[&(length - 2)].clone());
      
            let initial_phv = self.old_initial_phvs[&(length-2)].clone();
            mem::swap(&mut self.new_phvs, &mut self.old_phvs);
      
            mem::swap(&mut self.new_initial_phvs, &mut self.old_initial_phvs);
            (initial_phv, last_phv)
        }
    }
}


//For Debugging Purposes
impl fmt::Display for Pipeline{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s : String = String::from(""); 
        s.push_str( "Old Phvs: \n");
        for (key, value) in &self.old_phvs {
          s.push_str(&format!( "stage {} : \n{}\n", key, value));
        }
        s.push_str ("\nNew Phvs: \n");
        for (key_, value_) in &self.new_phvs {
          s.push_str( &format!("stage {} :  \n{}\n", key_, value_));
        }
        write!(f, "{}", s)
    }
}
