// Represents the Alu that's read from the
// .alu files

pub struct AluParsingUtils {

  pipeline_stage : i32,
  alu_count : i32,
  name : String,
  alu_body : String,
  is_stateful : bool,
  constant_vec : Vec<i32>,
  hole_configs_file : String,
  optimization_level : i32,
}

impl AluParsingUtils {

  pub fn new (t_pipeline_stage : i32,
              t_alu_count : i32,
              t_name : String,
              t_alu_body : String,
              t_is_stateful : bool,
              t_constant_vec : Vec <i32>,
              t_hole_configs_file : String,
              t_optimization_level : i32) -> Self {
    AluParsingUtils {
      pipeline_stage : t_pipeline_stage,
      alu_count : t_alu_count,
      name : t_name, 
      alu_body : t_alu_body,
      is_stateful : t_is_stateful,
      constant_vec : t_constant_vec,
      hole_configs_file : t_hole_configs_file,
      optimization_level : t_optimization_level
    }
  }
  
  // Returns the correct formatted string to be parsed
  // by LALRPOP
  pub fn prepend_opt_header_to_alu (&self) -> String{
    let name_header : String = 
        format! ("name : {}\n", self.name);
    let hole_configs_header : String =
        match self.hole_configs_file.is_empty() { 
          true => String::from("hole configs : \n"),
          false => format! ("hole configs : {}\n", self.hole_configs_file),
        };
    let pipeline_stage_header : String = 
        format! ("pipeline stage : {}\n", self.pipeline_stage);   
    let alu_count_header : String = 
        format! ("alu  : {}\n", self.alu_count);
    let opt_level_header : String = 
        format!("opt level : {}\n", self.optimization_level);
    let constant_vec_header : String = 
        format!("{:?}\n", self.constant_vec);
    format! ("{}{}{}{}{}{}{}", name_header, 
                               hole_configs_header,
                               pipeline_stage_header, 
                               alu_count_header, 
                               opt_level_header,
                               constant_vec_header,
                               self.alu_body) 
  }
  pub fn get_number_of_operands (&mut self) -> i32{
    let alu_body_line : Vec<String> = self.alu_body
                                        .split("\n")
                                        .map (|s| s.to_string())
                                        .collect();

    match self.is_stateful {
      true  => {
        let pkt_vec : Vec <&str> = alu_body_line [3].split(",").collect();
        pkt_vec.len() as i32
      },
      false => {  
        let pkt_vec : Vec <&str> = alu_body_line [4].split(",").collect();
        pkt_vec.len() as i32
      }
    }
  }
  pub fn get_number_of_state_variables (&self) -> i32 {
    match self.is_stateful {
      true => {
        let alu_body_line : Vec<String> = self.alu_body
                                              .split("\n")
                                              .map (|s| s.to_string())
                                              .collect();
        let state_vec : Vec <&str> = 
            alu_body_line [1].split(",").collect();
        state_vec.len() as i32
      },
      false => 0,
    }
  }
  pub fn increment_pipeline_stage (&mut self) {
    self.pipeline_stage += 1;
  }
  pub fn increment_alu_count (&mut self) {
    self.alu_count += 1;
  }
  pub fn reset_alu_count (&mut self) {
    self.alu_count = 0;
  }
  pub fn reset_alu_body (&mut self) {
    self.alu_count = 0;
  }
}
