use druzhba::phv_container::PhvContainer;
use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::alu::ALU;
use druzhba::input_mux::InputMux;
use druzhba::output_mux::OutputMux;use druzhba::phv::Phv;
use std::collections::HashMap;
pub fn name() -> String {
  "stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5".to_string()
}
pub fn pipeline_depth () -> i32 {
  4
}
pub fn pipeline_width () -> i32 {
  5
}
pub fn num_stateful_operands () -> i32 {
  2
}
pub fn num_stateless_operands () -> i32 {
  3
}
pub fn num_state_variables() -> i32 {
  1
}
pub fn num_stateful_alus() -> i32 {
  1
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_rel_op_0 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_0 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_const_0 (constant : i32) -> i32 {
  let constant_vec = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
  constant_vec[constant as usize]
}fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_1 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_const_1 (constant : i32) -> i32 {
  let constant_vec = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
  constant_vec[constant as usize]
}pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
    let old_state : Vec<i32> = state_vec.clone();
      if stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_rel_op_0(stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_0(state_vec[0], hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_0_global"]), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_0(phv_containers[0].get_value(), phv_containers[1].get_value(), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_const_0(hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_const_0_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_0_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_rel_op_0_global"]) != 0{
        state_vec[0] = stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_1(state_vec[0], hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Opt_1_global"])+stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_const_1(hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_const_1_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_Mux3_1_global"]);
      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_rel_op_0 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_0 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_const_0 (constant : i32) -> i32 {
  let constant_vec = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
  constant_vec[constant as usize]
}fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_1 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_const_1 (constant : i32) -> i32 {
  let constant_vec = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
  constant_vec[constant as usize]
}pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
    let old_state : Vec<i32> = state_vec.clone();
      if stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_rel_op_0(stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_0(state_vec[0], hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_0_global"]), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_0(phv_containers[0].get_value(), phv_containers[1].get_value(), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_const_0(hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_const_0_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_0_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_rel_op_0_global"]) != 0{
        state_vec[0] = stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_1(state_vec[0], hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Opt_1_global"])+stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_const_1(hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_const_1_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_Mux3_1_global"]);
      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_rel_op_0 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_0 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_const_0 (constant : i32) -> i32 {
  let constant_vec = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
  constant_vec[constant as usize]
}fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_1 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_const_1 (constant : i32) -> i32 {
  let constant_vec = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
  constant_vec[constant as usize]
}pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
    let old_state : Vec<i32> = state_vec.clone();
      if stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_rel_op_0(stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_0(state_vec[0], hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_0_global"]), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_0(phv_containers[0].get_value(), phv_containers[1].get_value(), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_const_0(hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_const_0_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_0_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_rel_op_0_global"]) != 0{
        state_vec[0] = stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_1(state_vec[0], hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Opt_1_global"])+stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_const_1(hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_const_1_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_Mux3_1_global"]);
      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_rel_op_0 (op1 : i32, op2 : i32, opcode : i32) -> i32{
  if opcode == 0 {
    (op1 != op2) as i32
  }
  else if opcode == 1{
    (op1 < op2) as i32
  }
  else if opcode == 2{
    (op1 > op2) as i32
  }
  else{
    (op1 == op2) as i32
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_0 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_const_0 (constant : i32) -> i32 {
  let constant_vec = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
  constant_vec[constant as usize]
}fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_1 (op : i32, enable : i32) -> i32 {
  if enable != 0 {
    0
  }
else{
  op
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{
  if ctrl == 0 {
    op1
  }
  else if ctrl == 1 {
    op2
  }
  else {
  op3
  }
}
fn stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_const_1 (constant : i32) -> i32 {
  let constant_vec = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
  constant_vec[constant as usize]
}pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
    let old_state : Vec<i32> = state_vec.clone();
      if stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_rel_op_0(stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_0(state_vec[0], hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_0_global"]), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_0(phv_containers[0].get_value(), phv_containers[1].get_value(), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_const_0(hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_const_0_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_0_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_rel_op_0_global"]) != 0{
        state_vec[0] = stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_1(state_vec[0], hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Opt_1_global"])+stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_const_1(hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_const_1_global"]), hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_Mux3_1_global"]);
      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let constant_vec : Vec <i32> = vec![0, 1, 2, 3, 102, 102, 1, 101, 203, 2, 3, 103];
      if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==0 {
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==1{
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==2{
        (vec![(phv_containers[0].get_value()+constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==3{
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==4{
        (vec![(phv_containers[0].get_value()-constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize]) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==5{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize]-phv_containers[0].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==6{
        (vec![(phv_containers[0].get_value()!=phv_containers[1].get_value()) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==7{
        (vec![((phv_containers[0].get_value()!=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==8{
        (vec![((phv_containers[0].get_value()==phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==9{
        (vec![((phv_containers[0].get_value()==constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==10{
        (vec![((phv_containers[0].get_value()>=phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==11{
        (vec![((phv_containers[0].get_value()>=constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==12{
        (vec![((phv_containers[0].get_value()<phv_containers[1].get_value())) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==13{
        (vec![((phv_containers[0].get_value()<constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize])) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==14{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(phv_containers[2].get_value()) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==15{
      if phv_containers[0].get_value()!=0 {
        (vec![(phv_containers[1].get_value()) as i32], Vec::new())
      }
        else{
        (vec![(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize]) as i32], Vec::new())
      }

      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==16{
        (vec![(((phv_containers[0].get_value()!=0)||(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==17{
        (vec![(((phv_containers[0].get_value()!=0)||(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize]!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==18{
        (vec![(((phv_containers[0].get_value()!=0)&&(phv_containers[1].get_value()!=0))) as i32], Vec::new())
      }
      else if hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_opcode"]==19{
        (vec![(((phv_containers[0].get_value()!=0)&&(constant_vec[hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_immediate"] as usize]!=0))) as i32], Vec::new())
      }
        else{
        (vec![(((phv_containers[0].get_value()==0))) as i32], Vec::new())
      }

    };
   Box::new(alu)
}
pub fn init_pipeline (hole_vars : HashMap <String, i32>) -> Pipeline { 
  let mut pipeline_stages : Vec<PipelineStage> = Vec::new();

  // Stage 0 stateful ALUs
  let mut stateful_alus_0 : Vec <ALU> = Vec::new();
  let mut stateless_alus_0 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, phv_containers : Vec::new(), state : Vec::new() };
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_0_ctrl"] });
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_0_0_1_ctrl"] });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_0_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_0_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_0_0 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0(hole_vars.clone()), state_variables : state_variables_0_0, input_muxes : stateful_input_muxes_0_0, output_mux : stateful_output_mux_0_0, is_stateful: true };
  stateful_alus_0.push(stateful_alu_0_0);

  // Stage 0 stateless ALUs
  let mut stateless_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux1_ctrl"] });
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux2_ctrl"] });
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0_mux3_ctrl"] });
  let stateless_output_mux_0_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_0_ctrl"]};
  let stateless_alu_0_0 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_0(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_0, output_mux : stateless_output_mux_0_0, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_0);
  let mut stateless_input_muxes_0_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux1_ctrl"] });
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux2_ctrl"] });
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1_mux3_ctrl"] });
  let stateless_output_mux_0_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_1_ctrl"]};
  let stateless_alu_0_1 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_1(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_1, output_mux : stateless_output_mux_0_1, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_1);
  let mut stateless_input_muxes_0_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux1_ctrl"] });
  stateless_input_muxes_0_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux2_ctrl"] });
  stateless_input_muxes_0_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2_mux3_ctrl"] });
  let stateless_output_mux_0_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_2_ctrl"]};
  let stateless_alu_0_2 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_2(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_2, output_mux : stateless_output_mux_0_2, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_2);
  let mut stateless_input_muxes_0_3 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux1_ctrl"] });
  stateless_input_muxes_0_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux2_ctrl"] });
  stateless_input_muxes_0_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3_mux3_ctrl"] });
  let stateless_output_mux_0_3 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_3_ctrl"]};
  let stateless_alu_0_3 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_3(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_3, output_mux : stateless_output_mux_0_3, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_3);
  let mut stateless_input_muxes_0_4 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux1_ctrl"] });
  stateless_input_muxes_0_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux2_ctrl"] });
  stateless_input_muxes_0_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4_mux3_ctrl"] });
  let stateless_output_mux_0_4 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_0_4_ctrl"]};
  let stateless_alu_0_4 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_0_4(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_4, output_mux : stateless_output_mux_0_4, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_4);
  let salu_configs_0 : Vec <i32> = vec![hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_salu_config_0_0"]];
  let output_mux_globals_0 : Vec <i32> = vec![hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_0_0_output_mux_global"]];
  let pipeline_stage_0 : PipelineStage = PipelineStage {stateful_alus : stateful_alus_0, stateless_alus : stateless_alus_0 , salu_configs : salu_configs_0, output_mux_globals : output_mux_globals_0, state_container : Vec::new()};
  pipeline_stages.push(pipeline_stage_0);

  // Stage 1 stateful ALUs
  let mut stateful_alus_1 : Vec <ALU> = Vec::new();
  let mut stateless_alus_1 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, phv_containers : Vec::new(), state : Vec::new() };
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_0_ctrl"] });
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_1_0_1_ctrl"] });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_1_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_1_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_1_0 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0(hole_vars.clone()), state_variables : state_variables_1_0, input_muxes : stateful_input_muxes_1_0, output_mux : stateful_output_mux_1_0, is_stateful: true };
  stateful_alus_1.push(stateful_alu_1_0);

  // Stage 1 stateless ALUs
  let mut stateless_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux1_ctrl"] });
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux2_ctrl"] });
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0_mux3_ctrl"] });
  let stateless_output_mux_1_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_0_ctrl"]};
  let stateless_alu_1_0 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_0(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_0, output_mux : stateless_output_mux_1_0, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_0);
  let mut stateless_input_muxes_1_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux1_ctrl"] });
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux2_ctrl"] });
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1_mux3_ctrl"] });
  let stateless_output_mux_1_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_1_ctrl"]};
  let stateless_alu_1_1 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_1(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_1, output_mux : stateless_output_mux_1_1, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_1);
  let mut stateless_input_muxes_1_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux1_ctrl"] });
  stateless_input_muxes_1_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux2_ctrl"] });
  stateless_input_muxes_1_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2_mux3_ctrl"] });
  let stateless_output_mux_1_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_2_ctrl"]};
  let stateless_alu_1_2 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_2(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_2, output_mux : stateless_output_mux_1_2, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_2);
  let mut stateless_input_muxes_1_3 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux1_ctrl"] });
  stateless_input_muxes_1_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux2_ctrl"] });
  stateless_input_muxes_1_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3_mux3_ctrl"] });
  let stateless_output_mux_1_3 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_3_ctrl"]};
  let stateless_alu_1_3 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_3(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_3, output_mux : stateless_output_mux_1_3, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_3);
  let mut stateless_input_muxes_1_4 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux1_ctrl"] });
  stateless_input_muxes_1_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux2_ctrl"] });
  stateless_input_muxes_1_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4_mux3_ctrl"] });
  let stateless_output_mux_1_4 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_1_4_ctrl"]};
  let stateless_alu_1_4 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_1_4(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_4, output_mux : stateless_output_mux_1_4, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_4);
  let salu_configs_1 : Vec <i32> = vec![hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_salu_config_1_0"]];
  let output_mux_globals_1 : Vec <i32> = vec![hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_1_0_output_mux_global"]];
  let pipeline_stage_1 : PipelineStage = PipelineStage {stateful_alus : stateful_alus_1, stateless_alus : stateless_alus_1 , salu_configs : salu_configs_1, output_mux_globals : output_mux_globals_1, state_container : Vec::new()};
  pipeline_stages.push(pipeline_stage_1);

  // Stage 2 stateful ALUs
  let mut stateful_alus_2 : Vec <ALU> = Vec::new();
  let mut stateless_alus_2 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_2_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, phv_containers : Vec::new(), state : Vec::new() };
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_0_ctrl"] });
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_2_0_1_ctrl"] });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_2_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_2_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_2_0 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0(hole_vars.clone()), state_variables : state_variables_2_0, input_muxes : stateful_input_muxes_2_0, output_mux : stateful_output_mux_2_0, is_stateful: true };
  stateful_alus_2.push(stateful_alu_2_0);

  // Stage 2 stateless ALUs
  let mut stateless_input_muxes_2_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux1_ctrl"] });
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux2_ctrl"] });
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0_mux3_ctrl"] });
  let stateless_output_mux_2_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_0_ctrl"]};
  let stateless_alu_2_0 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_0(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_0, output_mux : stateless_output_mux_2_0, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_0);
  let mut stateless_input_muxes_2_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux1_ctrl"] });
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux2_ctrl"] });
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1_mux3_ctrl"] });
  let stateless_output_mux_2_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_1_ctrl"]};
  let stateless_alu_2_1 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_1(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_1, output_mux : stateless_output_mux_2_1, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_1);
  let mut stateless_input_muxes_2_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux1_ctrl"] });
  stateless_input_muxes_2_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux2_ctrl"] });
  stateless_input_muxes_2_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2_mux3_ctrl"] });
  let stateless_output_mux_2_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_2_ctrl"]};
  let stateless_alu_2_2 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_2(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_2, output_mux : stateless_output_mux_2_2, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_2);
  let mut stateless_input_muxes_2_3 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux1_ctrl"] });
  stateless_input_muxes_2_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux2_ctrl"] });
  stateless_input_muxes_2_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3_mux3_ctrl"] });
  let stateless_output_mux_2_3 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_3_ctrl"]};
  let stateless_alu_2_3 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_3(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_3, output_mux : stateless_output_mux_2_3, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_3);
  let mut stateless_input_muxes_2_4 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux1_ctrl"] });
  stateless_input_muxes_2_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux2_ctrl"] });
  stateless_input_muxes_2_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4_mux3_ctrl"] });
  let stateless_output_mux_2_4 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_2_4_ctrl"]};
  let stateless_alu_2_4 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_2_4(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_4, output_mux : stateless_output_mux_2_4, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_4);
  let salu_configs_2 : Vec <i32> = vec![hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_salu_config_2_0"]];
  let output_mux_globals_2 : Vec <i32> = vec![hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_2_0_output_mux_global"]];
  let pipeline_stage_2 : PipelineStage = PipelineStage {stateful_alus : stateful_alus_2, stateless_alus : stateless_alus_2 , salu_configs : salu_configs_2, output_mux_globals : output_mux_globals_2, state_container : Vec::new()};
  pipeline_stages.push(pipeline_stage_2);

  // Stage 3 stateful ALUs
  let mut stateful_alus_3 : Vec <ALU> = Vec::new();
  let mut stateless_alus_3 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_3_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, phv_containers : Vec::new(), state : Vec::new() };
  stateful_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_0_ctrl"] });
  stateful_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_operand_mux_3_0_1_ctrl"] });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_3_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_3_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_3_0 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0(hole_vars.clone()), state_variables : state_variables_3_0, input_muxes : stateful_input_muxes_3_0, output_mux : stateful_output_mux_3_0, is_stateful: true };
  stateful_alus_3.push(stateful_alu_3_0);

  // Stage 3 stateless ALUs
  let mut stateless_input_muxes_3_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux1_ctrl"] });
  stateless_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux2_ctrl"] });
  stateless_input_muxes_3_0.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0_mux3_ctrl"] });
  let stateless_output_mux_3_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_0_ctrl"]};
  let stateless_alu_3_0 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_0(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_0, output_mux : stateless_output_mux_3_0, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_0);
  let mut stateless_input_muxes_3_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux1_ctrl"] });
  stateless_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux2_ctrl"] });
  stateless_input_muxes_3_1.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1_mux3_ctrl"] });
  let stateless_output_mux_3_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_1_ctrl"]};
  let stateless_alu_3_1 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_1(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_1, output_mux : stateless_output_mux_3_1, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_1);
  let mut stateless_input_muxes_3_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux1_ctrl"] });
  stateless_input_muxes_3_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux2_ctrl"] });
  stateless_input_muxes_3_2.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2_mux3_ctrl"] });
  let stateless_output_mux_3_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_2_ctrl"]};
  let stateless_alu_3_2 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_2(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_2, output_mux : stateless_output_mux_3_2, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_2);
  let mut stateless_input_muxes_3_3 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux1_ctrl"] });
  stateless_input_muxes_3_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux2_ctrl"] });
  stateless_input_muxes_3_3.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3_mux3_ctrl"] });
  let stateless_output_mux_3_3 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_3_ctrl"]};
  let stateless_alu_3_3 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_3(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_3, output_mux : stateless_output_mux_3_3, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_3);
  let mut stateless_input_muxes_3_4 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_3_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux1_ctrl"] });
  stateless_input_muxes_3_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux2_ctrl"] });
  stateless_input_muxes_3_4.push (InputMux { input_phv : empty_phv.clone(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4_mux3_ctrl"] });
  let stateless_output_mux_3_4 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_output_mux_phv_3_4_ctrl"]};
  let stateless_alu_3_4 : ALU = ALU {sequential_function : init_stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateless_alu_3_4(hole_vars.clone()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_3_4, output_mux : stateless_output_mux_3_4, is_stateful: false };
  stateless_alus_3.push(stateless_alu_3_4);
  let salu_configs_3 : Vec <i32> = vec![hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_salu_config_3_0"]];
  let output_mux_globals_3 : Vec <i32> = vec![hole_vars["stateful_fw_equivalent_3_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_stateful_alu_3_0_output_mux_global"]];
  let pipeline_stage_3 : PipelineStage = PipelineStage {stateful_alus : stateful_alus_3, stateless_alus : stateless_alus_3 , salu_configs : salu_configs_3, output_mux_globals : output_mux_globals_3, state_container : Vec::new()};
  pipeline_stages.push(pipeline_stage_3);

  // Initializing Pipeline using all PipelineStages 
  let pipeline : Pipeline = Pipeline::with_pipeline_stages(pipeline_stages);
  pipeline
}
