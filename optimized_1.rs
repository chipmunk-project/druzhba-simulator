use druzhba::phv_container::PhvContainer;
use druzhba::pipeline_stage::PipelineStage;
use druzhba::pipeline::Pipeline;
use druzhba::alu::ALU;
use druzhba::input_mux::InputMux;
use druzhba::output_mux::OutputMux;use druzhba::phv::Phv;
use std::collections::HashMap;
pub fn name() -> String {
  "times_two_sub_stateless_alu_arith_3_3".to_string()
}
pub fn pipeline_depth () -> i32 {
  3
}
pub fn pipeline_width () -> i32 {
  3
}
pub fn num_stateful_operands () -> i32 {
  2
}
pub fn num_stateless_operands () -> i32 {
  2
}
pub fn num_state_variables() -> i32 {
  1
}
pub fn num_stateful_alus() -> i32 {
  1
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 > op2) as i32
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Opt_0(op : i32) -> i32{
  op
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_0() -> i32{
  2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Opt_1(op : i32) -> i32{
  op
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_1() -> i32{
  5
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_2() -> i32{
  2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Opt_2(op : i32) -> i32{
  0
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_3() -> i32{
  3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_4() -> i32{
  5
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_rel_op_0(times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Opt_0(state_vec[0]), times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_0(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_0())) != 0{
        state_vec[0] = times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Opt_1(state_vec[0])+times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_arith_op_0(times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_1()), times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_2()));
      }
        else{
        state_vec[0] = times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Opt_2(state_vec[0])+times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_arith_op_1(times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_3(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_3()), times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0_const_4()));
      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_0_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_0_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()+phv_containers[1].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_0_2(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()-5) as i32], Vec::new())

    };
   Box::new(alu)
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Opt_0(op : i32) -> i32{
  0
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_0() -> i32{
  3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Opt_1(op : i32) -> i32{
  0
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_1() -> i32{
  5
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_2() -> i32{
  3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Opt_2(op : i32) -> i32{
  0
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_3() -> i32{
  4
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_4() -> i32{
  3
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_rel_op_0(times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Opt_0(state_vec[0]), times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_0(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_0())) != 0{
        state_vec[0] = times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Opt_1(state_vec[0])+times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_arith_op_0(times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_1()), times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_2()));
      }
        else{
        state_vec[0] = times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Opt_2(state_vec[0])+times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_arith_op_1(times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_3(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_3()), times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0_const_4()));
      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_1_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()-phv_containers[1].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_1_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(2-phv_containers[0].get_value()) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_1_2(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()-5) as i32], Vec::new())

    };
   Box::new(alu)
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_rel_op_0(op1 : i32, op2 : i32) -> i32{
  (op1 == op2) as i32
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Opt_0(op : i32) -> i32{
  0
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_0(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op1
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_0() -> i32{
  5
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Opt_1(op : i32) -> i32{
  0
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_arith_op_0(op1 : i32, op2 : i32) -> i32{
  op1 + op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_1(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_1() -> i32{
  2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_2(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_2() -> i32{
  3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Opt_2(op : i32) -> i32{
  0
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_arith_op_1(op1 : i32, op2 : i32) -> i32{
  op1 - op2
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_3(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_3() -> i32{
  4
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_4(op1 : i32, op2 : i32, op3 : i32) -> i32{
  op3
}
fn times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_4() -> i32{
  4
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
    let old_state : Vec<i32> = state_vec.clone();
      if times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_rel_op_0(times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Opt_0(state_vec[0]), times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_0(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_0())) != 0{
        state_vec[0] = times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Opt_1(state_vec[0])+times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_arith_op_0(times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_1(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_1()), times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_2(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_2()));
      }
        else{
        state_vec[0] = times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Opt_2(state_vec[0])+times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_arith_op_1(times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_3(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_3()), times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_Mux3_4(phv_containers[0].get_value(), phv_containers[1].get_value(), times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0_const_4()));
      }

    (old_state, state_vec.clone())
    };
    Box::new(alu)
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_2_0(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()+2) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_2_1(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(phv_containers[0].get_value()+5) as i32], Vec::new())

    };
   Box::new(alu)
}
pub fn init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_2_2(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{
    // state_vec unused
    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){
        (vec![(2) as i32], Vec::new())

    };
   Box::new(alu)
}
//_h not used
pub fn init_pipeline (_h : HashMap <String, i32>) -> Pipeline { 
  let mut pipeline_stages : Vec<PipelineStage> = Vec::new();

  // Stage 0 stateful ALUs
  let mut stateful_alus_0 : Vec <ALU> = Vec::new();
  let mut stateless_alus_0 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, phv_containers : Vec::new(), state : Vec::new() };
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateful_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_0_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_0_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_0_0 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateful_alu_0_0(HashMap::new()), state_variables : state_variables_0_0, input_muxes : stateful_input_muxes_0_0, output_mux : stateful_output_mux_0_0, is_stateful: true };
  stateful_alus_0.push(stateful_alu_0_0);

  // Stage 0 stateless ALUs
  let mut stateless_input_muxes_0_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 3 });
  stateless_input_muxes_0_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_0_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_0_0 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_0_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_0, output_mux : stateless_output_mux_0_0, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_0);
  let mut stateless_input_muxes_0_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_0_1.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  let stateless_output_mux_0_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_0_1 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_0_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_1, output_mux : stateless_output_mux_0_1, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_1);
  let mut stateless_input_muxes_0_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_0_2.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  stateless_input_muxes_0_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_0_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 0};
  let stateless_alu_0_2 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_0_2(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_0_2, output_mux : stateless_output_mux_0_2, is_stateful: false };
  stateless_alus_0.push(stateless_alu_0_2);
  let salu_configs_0 : Vec <i32> = vec![0];
  let output_mux_globals_0 : Vec <i32> = vec![0];
  let pipeline_stage_0 : PipelineStage = PipelineStage {stateful_alus : stateful_alus_0, stateless_alus : stateless_alus_0 , salu_configs : salu_configs_0, output_mux_globals : output_mux_globals_0, state_container : Vec::new() };
  pipeline_stages.push(pipeline_stage_0);

  // Stage 1 stateful ALUs
  let mut stateful_alus_1 : Vec <ALU> = Vec::new();
  let mut stateless_alus_1 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, phv_containers : Vec::new(), state : Vec::new() };
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_1_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_1_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_1_0 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateful_alu_1_0(HashMap::new()), state_variables : state_variables_1_0, input_muxes : stateful_input_muxes_1_0, output_mux : stateful_output_mux_1_0, is_stateful: true };
  stateful_alus_1.push(stateful_alu_1_0);

  // Stage 1 stateless ALUs
  let mut stateless_input_muxes_1_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  stateless_input_muxes_1_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_1_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_1_0 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_1_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_0, output_mux : stateless_output_mux_1_0, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_0);
  let mut stateless_input_muxes_1_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_1_1.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  let stateless_output_mux_1_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_1_1 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_1_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_1, output_mux : stateless_output_mux_1_1, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_1);
  let mut stateless_input_muxes_1_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_1_2.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_1_2.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  let stateless_output_mux_1_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_1_2 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_1_2(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_1_2, output_mux : stateless_output_mux_1_2, is_stateful: false };
  stateless_alus_1.push(stateless_alu_1_2);
  let salu_configs_1 : Vec <i32> = vec![0];
  let output_mux_globals_1 : Vec <i32> = vec![0];
  let pipeline_stage_1 : PipelineStage = PipelineStage {stateful_alus : stateful_alus_1, stateless_alus : stateless_alus_1 , salu_configs : salu_configs_1, output_mux_globals : output_mux_globals_1, state_container : Vec::new() };
  pipeline_stages.push(pipeline_stage_1);

  // Stage 2 stateful ALUs
  let mut stateful_alus_2 : Vec <ALU> = Vec::new();
  let mut stateless_alus_2 : Vec <ALU> = Vec::new();
  let mut stateful_input_muxes_2_0 : Vec<InputMux> = Vec::new();
  let empty_phv : Phv <i32> = Phv { bubble : true, phv_containers : Vec::new(), state : Vec::new() };
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateful_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  // No hole variables for stateful ALU OutputMux
  let stateful_output_mux_2_0 : OutputMux = OutputMux {input_phv_containers : Vec::new(), index : 0 };
  let state_variables_2_0 : Vec<i32> = vec![0; 1];
  let stateful_alu_2_0 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateful_alu_2_0(HashMap::new()), state_variables : state_variables_2_0, input_muxes : stateful_input_muxes_2_0, output_mux : stateful_output_mux_2_0, is_stateful: true };
  stateful_alus_2.push(stateful_alu_2_0);

  // Stage 2 stateless ALUs
  let mut stateless_input_muxes_2_0 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  stateless_input_muxes_2_0.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  let stateless_output_mux_2_0 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_2_0 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_2_0(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_0, output_mux : stateless_output_mux_2_0, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_0);
  let mut stateless_input_muxes_2_1 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 2 });
  stateless_input_muxes_2_1.push (InputMux { input_phv : empty_phv.clone(), index : 1 });
  let stateless_output_mux_2_1 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 1};
  let stateless_alu_2_1 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_2_1(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_1, output_mux : stateless_output_mux_2_1, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_1);
  let mut stateless_input_muxes_2_2 : Vec<InputMux> = Vec::new();
  stateless_input_muxes_2_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  stateless_input_muxes_2_2.push (InputMux { input_phv : empty_phv.clone(), index : 0 });
  let stateless_output_mux_2_2 : OutputMux = OutputMux { input_phv_containers : Vec::new(), index : 0};
  let stateless_alu_2_2 : ALU = ALU {sequential_function : init_times_two_sub_stateless_alu_arith_3_3_stateless_alu_2_2(HashMap::new()), state_variables : Vec::new(), input_muxes : stateless_input_muxes_2_2, output_mux : stateless_output_mux_2_2, is_stateful: false };
  stateless_alus_2.push(stateless_alu_2_2);
  let salu_configs_2 : Vec <i32> = vec![1];
  let output_mux_globals_2 : Vec <i32> = vec![0];
  let pipeline_stage_2 : PipelineStage = PipelineStage {stateful_alus : stateful_alus_2, stateless_alus : stateless_alus_2 , salu_configs : salu_configs_2, output_mux_globals : output_mux_globals_2, state_container : Vec::new() };
  pipeline_stages.push(pipeline_stage_2);

  // Initializing Pipeline using all PipelineStages 
  let pipeline : Pipeline = Pipeline::with_pipeline_stages(pipeline_stages);
  pipeline
}
