use std::fmt;

use std::fs;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
  // Need a rw lock because Rust does not like
  // aliasing + mutation at the same time. The
  // rw lock provides the necessary synchronization
  static ref HELPER_STRING: RwLock<String> = 
    RwLock::new(String::from(""));
  static ref HOLE_VALS : RwLock <HashMap <String, i32>> =
    RwLock::new(HashMap::new());
  static ref OPTIMIZED : RwLock <i32> = 
      RwLock::new(0);
  static ref STATE_VAR_MAP : RwLock <HashMap <String, i32>> =
    RwLock::new(HashMap::new());
  static ref HOLE_VAR_MAP : RwLock <HashMap <String, i32>> =
    RwLock::new(HashMap::new());
  static ref PHV_CONTAINER_MAP : RwLock <HashMap <String, i32>> =
    RwLock::new(HashMap::new());
  static ref PIPELINE_STAGE : RwLock <i32> = 
      RwLock::new(0);
  static ref ALU_NUMBER : RwLock <i32> = 
      RwLock::new(0);
  static ref NAME : RwLock <String> =
      RwLock::new(String::from(""));
  static ref FUNC_COUNT : RwLock <HashMap <String, i32> >=
      RwLock::new (HashMap::new());
  static ref CONSTANT_VEC : RwLock <Vec <String> >=
      RwLock::new(Vec::new());
}

pub enum Alu {
  Program ( Option<Box<OptHeader>>, Box <Header>, Box <Stmt>),
}
impl fmt::Display for Alu {
  fn fmt (&self, f : &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      Alu::Program (opt_header, header, stmt) => {
        { // Write lock in its own scope to avoid deadlock
          let mut tmp = HELPER_STRING.write().unwrap();
          *tmp = String::from("");
          STATE_VAR_MAP.write().unwrap().clear();
          HOLE_VAR_MAP.write().unwrap().clear();
          PHV_CONTAINER_MAP.write().unwrap().clear();
          *PIPELINE_STAGE.write().unwrap() = 0;
          NAME.write().unwrap().clear();
          FUNC_COUNT.write().unwrap().clear();
          CONSTANT_VEC.write().unwrap().clear();
          HOLE_VALS.write().unwrap().clear();
          *OPTIMIZED.write().unwrap() = 0;
    
        }
        
        match opt_header {
          Some (h) => write!(f, "{}", h),
          None     => write!(f, ""),
        }.expect ("Error: issue with match statement on OptHeader");
        let constant_vec_string : String = 
            match *OPTIMIZED.read().unwrap() {
                0 => {
                    let temp_constant_vec = CONSTANT_VEC.read()
                        .unwrap()
                        .clone();
                    let mut constant_vec = Vec::new();
                    for x in temp_constant_vec.iter(){
                      constant_vec.push (x.parse::<i32>().unwrap());
                    }
                    match temp_constant_vec.len() == 0 {
                        true => "".to_string(),
                        false => format!("    let constant_vec : Vec <i32> = vec!{:?};\n",
                            constant_vec),
                    }
                },
                _ => String::from(""),
            };
        let body : String = String::from(&format!("{}{}", header, stmt));
        let state_var_length = STATE_VAR_MAP.read().unwrap().len();
        let mut end : String = String::from("");
        
        if state_var_length == 0 {
          end.push_str("    };\n   Box::new(alu)\n}\n"); 
        }
        else {
          end.push_str("    (old_state, state_vec.clone())\n    };\n    Box::new(alu)\n}\n");
        }

        // Function name to initialize ALU function
        let init_name : String = 
            format! ("init_{}_{}_{}_{}", 
                &NAME.read().unwrap().to_string(),
                match FUNC_COUNT.read().unwrap()["state"] {
                  0 => "stateful_alu",
                  1 => "stateless_alu",
                  _ => panic!("Error: invalid state indicator"),
                },
                *PIPELINE_STAGE.read().unwrap(),
                *ALU_NUMBER.read().unwrap() );

        // The function inside of the initialize ALU function that
        // will be returned
        let inner_header : String = 

            match FUNC_COUNT.read().unwrap()["state"] {
                0 => String::from ("    let alu = move |state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){\n"),
                1 =>  String::from ("    // state_vec unused\n    let alu = move |_state_vec : &mut Vec <i32>, phv_containers : &Vec <PhvContainer <i32>>| -> (Vec <i32>, Vec <i32>){\n"),
                _ => panic!("Error: invalid state indicator"),
            };

        let mut outer_header : String = String::from("pub fn ");
        outer_header.push_str (&init_name);
        outer_header.push_str (
            match *OPTIMIZED.read().unwrap(){
              0 => "(hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{\n",
              _ => "(_hole_vars : HashMap <String, i32>) -> Box <dyn Fn (&mut Vec <i32>, &Vec <PhvContainer <i32>>) -> (Vec <i32>, Vec <i32> ) >{\n",
            });

        
        write!(f, "{}{}{}{}{}", outer_header, 
                                inner_header, 
                                constant_vec_string,
                                body, 
                                end)
      },
    }
  }
}
pub enum OptHeader {
  Data (String,  // Sketch name
        Option<String>,// Hole config file
        i32, // Pipeline stage
        i32, // ALU number
        i32, // opt_level
        Vec<String>)
}

impl fmt::Display for OptHeader {

  fn fmt (&self, f : &mut fmt::Formatter) -> fmt::Result {
    match *&self {
      OptHeader::Data (name, 
                       hole_config_file,
                       stage, 
                       alu_num, 
                       opt_level,
                       constant_vec) => {
          
        *NAME.write().unwrap() = name.to_string();
        *PIPELINE_STAGE.write().unwrap() = *stage;       
        *ALU_NUMBER.write().unwrap() = *alu_num;
        *CONSTANT_VEC.write().unwrap() = constant_vec.to_vec();
        *OPTIMIZED.write().unwrap() = *opt_level;
        *HOLE_VALS.write().unwrap() = 
            match hole_config_file {
              Some (f) => {
                assert!(*opt_level != 0);
                extract_hole_cfgs(f.to_string())
              },
              _        => {

                assert!(*opt_level == 0);
                HashMap::new()
              },
          };

        write!(f, "")
     }
    }
  }
}
pub enum Header {
  InputData (String, // "stateful" or "stateless"
             Vec<String>, // State variables
             Vec<String>, // Hole variables
             Vec<String>), // Phv containers

}
impl fmt::Display for Header {
  fn fmt (&self, f : &mut fmt::Formatter) -> fmt::Result {
    match &*self{
      Header::InputData (s, v1, v2, v3) => {
        if s == "stateless" {
          FUNC_COUNT.write().unwrap().insert("state".to_string(), 1);
        }
        else if s == "stateful"{

          FUNC_COUNT.write().unwrap().insert("state".to_string(), 0);
        }
        
        if s == "stateless" && v1.len() > 0 {
          panic! ("State variables given to statelss ALU");
        }
        if v1.len() > 0 {
          for i in 0..v1.len(){
            STATE_VAR_MAP.write().unwrap().insert( v1[i].clone(), i as i32);
          }
        }
        if v2.len() > 0{
          for i in 0..v2.len(){
            HOLE_VAR_MAP.write().unwrap().insert( v2[i].clone(), i as i32);
          }
        }
        if v3.len() > 0 {
          for i in 0..v3.len() {
            PHV_CONTAINER_MAP.write().unwrap().insert(v3[i].clone(), 
                                                      i as i32);
            FUNC_COUNT.write().unwrap().insert (v3[i].clone(), 0);
          }
        }
        if v1.len() > 0 {
          write!(f, "    let old_state : Vec<i32> = state_vec.clone();\n")
        }
        else {
          write!(f, "")
        }
      },
    }
  }
}
pub enum Stmt {
  Return (Box <Expr>), 
  If (Box<Expr>, // If expr
    Vec <Box<Stmt>>, // If body
    Vec<(Box<Expr>, Vec <Box<Stmt>>)>, // Pairs of elif expr along
                                       // with expr statements
    Option<Vec<Box<Stmt>>>), // Else body
  Assign (Box<Expr>, Box<Expr>), // Variable assigned to an expr
}

impl fmt::Display for Stmt {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Need to borrow self so we don't take ownership
    // of data in tuple
    match &*self{
      Stmt::Return (e) => 
          write!(f, "        (vec![({}) as i32], Vec::new())\n", e),

      // Generates if statement
      Stmt::If (e1, if_block, elif_block, else_block) => {
          let mut if_body : String = String::from("");
          let mut elif_bodies : String = String::from("");
          let mut else_body : String = String::from("");
          let is_stateful : bool = 
              match FUNC_COUNT.read().unwrap()["state"]{
                0 => true,
                1 => false,

                _ => panic! ("Error: incorrect code given to state indicator"), 
              };
          let opt_inequality : String = 
              match FUNC_COUNT.read().unwrap()["state"]{
                  0 => String::from("!= 0"),
                  1 => String::from(""),
                  _ => panic! ("Error: incorrect code given to state indicator"), 
          };

          let expr : &str = &format!("{}",e1);
          let optimize : i32 = *OPTIMIZED.read().unwrap();
          if is_stateful || optimize == 0  || !(expr.contains("opcode") && expr.contains("hole_vars")) {
              if_body.push_str (&format!("      if {} {}{{\n", expr, 
                                         opt_inequality));
              for stmt in if_block{
                if_body.push_str (&stmt.to_string());
              }
              if_body.push_str(&"      }\n".to_string());
              for (expr, stmts) in elif_block{
                elif_bodies.push_str (&"      else if ".to_string());
                elif_bodies.push_str(&expr.to_string());
                elif_bodies.push_str(&format!("{}{{\n", opt_inequality));
                for stmt in stmts{
                  elif_bodies.push_str(&stmt.to_string());
                }
                elif_bodies.push_str(&"      }\n".to_string());
              }
              else_body= match else_block {
                Some(else_block_stmts) => {
                  let mut tmp_str : String = String::from("        else{\n");
                  for stmt in else_block_stmts{
                    tmp_str.push_str(&stmt.to_string());
                  }
                  tmp_str.push_str(&"      }\n".to_string());
                  tmp_str
                }
                _         => String::from("")
              };
          }
          // Optimized stateless ALU
          else{
              let name : String = (&*NAME.read().unwrap()).to_string();
              let start_bytes = expr.find(&name).unwrap_or(0);
              let end_bytes = expr.find("opcode").unwrap_or(expr.len());
              let result : &str = &format!("{}opcode", &expr[start_bytes..end_bytes]);
              let opcode : i32 = 
                  match HOLE_VALS.read().unwrap().get(result){
                     Some (num) => *num,
                     _          => panic!("Error: Could not access hole variable by hole name"),
                  };
              let mut found_branch : bool = false;
              if opcode == 0 {
                for stmt in if_block {
                  if_body.push_str (&stmt.to_string());
                }
                found_branch = true;
              }
              let mut index : i32 = 1;
              for (_, stmts) in elif_block {
                if opcode == index {
                  for stmt in stmts {
                    elif_bodies.push_str(&stmt.to_string());
                  }
                  found_branch = true;
                  break;
                }
                index += 1;
              }
              if !found_branch {
                else_body = match else_block {
                  Some(else_block_stmts) => {
                    let mut tmp_str : String = "".to_string();
                    for stmt in else_block_stmts{
                      tmp_str.push_str(&stmt.to_string());
                    }
                    tmp_str
                  }
                  _         => String::from("")
                };
              }

            }
            
          write!(f, "{}{}{}\n",
                 if_body, elif_bodies, else_body)
      },
      // TODO: Possibly check if the var name is an invalid expr.
      // Otherwise, just wait until rustc catches it
      Stmt::Assign (s, e) => write!(f, "        {} = {};\n", s, e),
    }
  }
}

pub enum Expr {
  // Captures parenthesis
  ExprWithParen (Box <Expr>),
  Num (i32),
  Var (String),
  Op (Box <Expr>, Opcode, Box<Expr> ),
  Mux3 (Box <Expr>, Box <Expr>, Box <Expr> ),
  Mux2 (Box <Expr>, Box <Expr>),
  Opt (Box <Expr>),
  ArithOp (Box<Expr>, Box<Expr>),
  Relop (Box<Expr>, Box<Expr>),
  Constant,
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    // Updates the FUNC_COUNT HashMap to keep track of how many
    // of each helper function is in the ALU. These counters
    // are used to create the hole variable names and function
    // values
    let update_func_count = | key : String | {

      let mut func_count_ref = FUNC_COUNT.write().unwrap();
      match func_count_ref.get(&key){
        Some (&count) => func_count_ref.insert (key.clone(), count + 1),
        _             => func_count_ref.insert (key.clone(), 0),
      };
    };
    // Generates the hole name for a specific ALU helper function
    // or for a hole variable
    let generate_hole_name = 
        | mut helper_name : String| -> String {
            
      update_func_count (helper_name.clone());
      let helper_num : i32 = FUNC_COUNT.read().unwrap() [&helper_name];
      let pipeline : i32 = *PIPELINE_STAGE.read().unwrap();
      let alu : i32 = *ALU_NUMBER.read().unwrap();
      let name : String = NAME.read().unwrap().clone();
      if helper_name == "immediate_operand" {
        helper_name = "immediate".to_string();
      }
      match FUNC_COUNT.read().unwrap()["state"] {
        0 => format!("{}_stateful_alu_{}_{}_{}_{}_global", name, pipeline, alu, helper_name, helper_num),
        // TODO: Support a global format in case a stateless ALU
        // has helper functions
        1 => format!("{}_stateless_alu_{}_{}_{}", name, pipeline, alu, helper_name),
        _ => panic! ("Error: incorrect code given to state indicator"),
      }
    }; 
    // Generates the function name for a helper function
    let generate_helper_name =
        | helper_name : String| -> String {
  
        let helper_num : i32 = FUNC_COUNT.read().unwrap() [&helper_name];
        let pipeline : i32 = *PIPELINE_STAGE.read().unwrap();
        let alu : i32 = *ALU_NUMBER.read().unwrap();
        let name : String = NAME.read().unwrap().clone();

        match FUNC_COUNT.read().unwrap()["state"] {
          0 => format!("{}_stateful_alu_{}_{}_{}_{}", name, pipeline, alu, helper_name, helper_num),
          // TODO: Stateless ALU specs should not be creating any
          // helpers like in Chipmunk so either change this to 
          // support a scenario in which that happens or panic
          1 => format!("{}_stateless_alu_{}_{}_{}", name, pipeline, alu, helper_name),
          _ => panic! ("Error: incorrect code given to state indicator"),
          
      }
    };

    // Takes in an identifier and checks whether it is valid variable
    // and generates code to access that value. Since state variables
    // are in a vector, it figures out which vector index that
    // variable corresponds to. The same is done with phv_containers.
    // For hole variables, make a call to generate_hole_name
    let find_var_name = | variable : String| -> String {
      let optimize : i32 = *OPTIMIZED.read().unwrap();
      match STATE_VAR_MAP.read().unwrap().get(&variable){
        Some (ind1) => format!("state_vec[{}]",ind1),
        _           => {
          match PHV_CONTAINER_MAP.read().unwrap().get(&variable){
            Some (ind2) => format!("phv_containers[{}].get_value()",ind2),
            _           => {
              match HOLE_VAR_MAP.read().unwrap().get(&variable){
                Some (_ind3) => {
                    let hole_name : String = generate_hole_name (variable.clone());
                      if hole_name.contains("immediate") {
                        let tmp_const_vec = CONSTANT_VEC.read().unwrap();
                        if optimize == 0 {
                          let name = generate_hole_name(variable.clone());
                          match tmp_const_vec.len() == 0 {
                              true => format!("hole_vars[\"{}\"]", 
                                  name),
                              false => format!("constant_vec[hole_vars[\"{}\"] as usize]", 
                                  name),
                          }
                        
                        }
                        // Replace immediate with correct constant
                        // vector value in stateless ALU
                        else {
                          let name = generate_hole_name (variable.clone());
                          let hole_val = 
                            match HOLE_VALS.read().unwrap().get(&name){
                              Some (num) => *num,
                              _          => panic!("Error: Could not get hole variable"),

                          };
                          let temp_constant_vec = CONSTANT_VEC
                              .read()
                              .unwrap();
                          match temp_constant_vec.len() == 0 {
                              true => format!("{}", hole_val),
                              false => format!("{}", temp_constant_vec[hole_val as usize]),
                          } 
                        }
                      }
                      else {
                        format!("hole_vars[\"{}\"]", generate_hole_name (variable.clone()))
                      }
                     
                },
                
                _            => panic! ("Error: variable '{}' is undefined", variable),

              }
            }, 
          }
        },
      }
    };
    let optimize : i32 = *OPTIMIZED.read().unwrap();
    match &*self{
      Expr::ExprWithParen (e) => 
          write!(f, "({})",e),
      Expr::Num (n)=> write! (f, "{}",n),
      Expr::Var (v)=> write! (f, "{}", find_var_name(v.to_string())),
      Expr::Op (e1, op, e2)=> 
          write! (f, "{}{}{}", e1, op, e2),
       
      // Generates the calls to the helper functions and the 
      // helper functions themselves
      Expr::Mux3 (e1, e2, e3) => { 
          let mux3_hole_name : String = 
              generate_hole_name ("Mux3".to_string());
          let mux3_name : String =
              generate_helper_name ("Mux3".to_string());

          // Unoptimized case: generate function call
          // and use generate_mux3 to generate the mux3
          // function
          if optimize == 0 {
            generate_mux3(mux3_name.clone());
            write! (f, "{}({}, {}, {}, hole_vars[\"{}\"])",
                    mux3_name,
                    e1, e2, e3, mux3_hole_name)

          }
          else if optimize > 0 {
            let opcode : i32 = 
                match HOLE_VALS.read().unwrap().get(&mux3_hole_name){
                   Some (num) => *num,
                   _          => panic!("Error: Could not access mux3 hole variable by hole name"),
                };
            // Optimize the mux3 function using the hole variable
            if optimize == 1 {
                generate_mux3_optimized(mux3_name.clone(),
                                        opcode);
                write! (f, "{}({}, {}, {})",
                        mux3_name,
                        e1, e2, e3)
            }
            // Get rid of the mux3 function altogether
            // and replace the funcion call with the
            // original mux3 function body using in-lining
            else {
                let s1 : String = format!("{}", e1);
                let s2 : String = format!("{}", e2);
                let s3 : String = format!("{}", e3);
                match opcode {
                    0 => write!(f, "{}", s1),
                    1 => write!(f, "{}", s2),
                    _ => write!(f, "{}", s3)
                }
            }
          }

          else {
            panic!("Error: Optimization level is invalid");
          }
      },
      Expr::Mux2 (e1, e2) => {
          let mux2_hole_name : String = 
              generate_hole_name ("Mux2".to_string());

          let mux2_name : String = 
              generate_helper_name ("Mux2".to_string());

          if optimize == 0 {
            generate_mux2(mux2_name.clone());
            write! (f, "{}({}, {}, hole_vars[\"{}\"])", 
                    mux2_name,
                    e1, e2, mux2_hole_name)
          }
          else if optimize > 0{
            let opcode : i32 = 
                match HOLE_VALS.read().unwrap().get(&mux2_hole_name){
                   Some (num) => *num,
                   _          => panic!("Error: Could not access mux 2 hole variable by hole name"),
                };
            if optimize == 1 {
                generate_mux2_optimized(mux2_name.clone(),
                                        opcode);
                write! (f, "{}({}, {})",
                        mux2_name,
                        e1, e2)
            }
            else {
                let s1 : String = format!("{}", e1);
                let s2 : String = format!("{}", e2);

                match opcode {
                    0 => write!(f, "{}", s1),
                    _ => write!(f, "{}", s2),
                }
            }

          }
          else { 
            panic!("Error: Optimization level is invalid");
          }

      },
      Expr::Opt (e) => {
          let opt_hole_name : String = 
              generate_hole_name ("Opt".to_string());
          let opt_name : String = 
              generate_helper_name ("Opt".to_string());
          if optimize == 0 {
            generate_opt(opt_name.clone());
            write! (f, "{}({}, hole_vars[\"{}\"])", 
                  opt_name,
                  e, opt_hole_name)
          }
          else if optimize > 0 {
             let opcode : i32 = 
                match HOLE_VALS.read().unwrap().get(&opt_hole_name){
                   Some (num) => *num,
                   _          => panic!("Error: Could not access opt hole variable by hole name {}", opt_hole_name),
                };
            if optimize == 1 {
                generate_opt_optimized(opt_name.clone(),
                                       opcode);
                write! (f, "{}({})",
                        opt_name,
                        e)
            }
            else {
                match opcode {
                    0 => write!(f, "{}", e),
                    _ => write!(f, "0"),
                }
            }
           
          }
          else { 
            panic!("Error: Optimization level is invalid");
          }

      },
      Expr::ArithOp (e1, e2) => {
          let arith_op_hole_name : String = 
              generate_hole_name ("arith_op".to_string());

        let arith_op_name : String = 
            generate_helper_name ("arith_op".to_string());
        if optimize == 0{
          generate_arith_op(arith_op_name.clone());
          write! (f, "{} ({}, {}, hole_vars[\"{}\"])", 
                  arith_op_name,
                  e1, e2, arith_op_hole_name)
        }
        else if optimize > 0{
           let opcode : i32 = 
              match HOLE_VALS.read().unwrap().get(&arith_op_hole_name){
                 Some (num) => *num,
                 _          => panic!("Error: Could not access arith op hole variable by hole name"),
              };
           if optimize == 1 {
             generate_arith_op_optimized(arith_op_name.clone(),
                                         opcode);
             write! (f, "{}({}, {})",
                     arith_op_name,
                     e1, e2)
           }
           else {
             match opcode {
                0 => write!(f, "({} + {})", e1, e2),
                _ => write!(f, "({} - {})", e1, e2),

             }
           }
        }
        else { 
          panic!("Error: Optimization level is invalid");
        }

      },
      Expr::Relop (e1, e2) => {
        let rel_op_hole_name : String = 
            generate_hole_name ("rel_op".to_string());

        let rel_op_name : String = 
            generate_helper_name ("rel_op".to_string());
        if optimize == 0{
          generate_rel_op(rel_op_name.clone());
          write! (f, "{}({}, {}, hole_vars[\"{}\"])", 
                  rel_op_name,
                  e1, e2, rel_op_hole_name)
        }
        else if optimize > 0{
           let opcode : i32 = 
              match HOLE_VALS.read().unwrap().get(&rel_op_hole_name){
                 Some (num) => *num,
                 _          => panic!("Error: Could not access rel op hole variable by hole name"),
              };
           if optimize == 1 {
              generate_rel_op_optimized(rel_op_name.clone(),
                                        opcode);
              write! (f, "{}({}, {})",
                      rel_op_name,
                      e1, e2)
           }
           else {
              match opcode {
                0 => write!(f, "(({} != {}) as i32)", e1, e2),
                1 => write!(f, "(({} < {}) as i32)", e1, e2),
                2 => write!(f, "(({} > {}) as i32)", e1, e2),
                _ => write!(f, "(({} == {}) as i32)", e1, e2),
              }
           }
        }
        else { 
          panic!("Error: Optimization level is invalid");
        }
      },
      Expr::Constant => {
        let constant_hole_name : String = 
            generate_hole_name ("const".to_string());

        let constant_name : String = 
            generate_helper_name ("const".to_string());
        if optimize == 0{
            generate_constant(constant_name.clone());
            write!(f, "{}(hole_vars[\"{}\"])", 
                   constant_name,
                   constant_hole_name)
        }
        else if optimize > 0 {
           let opcode : i32 = 
              match HOLE_VALS.read().unwrap().get(&constant_hole_name){
                 Some (num) => *num,
                 _          => panic!("Error: Could not access constant hole variable by hole name"),
              };
            if optimize == 1 { 
                generate_constant_optimized(constant_name.clone(),
                                            opcode);
                write! (f, "{}()",
                      constant_name)
            }
            else {
                generate_constant_optimized(constant_name.clone(),
                    opcode);
                let temp_constant_vec : Vec <String> = CONSTANT_VEC
                     .read()
                     .unwrap()
                     .clone();

               match opcode >= temp_constant_vec.len() as i32 &&
                     temp_constant_vec.len() > 0 {
                  true => write!(f, 
                       "{}",
                       temp_constant_vec[temp_constant_vec.len()-1]),
                  false => match temp_constant_vec.len() > 0 { 
                      true => 
                          write!(f, 
                              "{}", 
                              temp_constant_vec[opcode as usize]),
                      false => write!(f, "{}", opcode),
                    }

                }
            }
        }
        else { 
          panic!("Error: Optimization level is invalid");
        }


      },
    }
  }
}
pub enum Opcode {
  Mul,
  Div,
  Add,
  Sub,
  Equal,
  Greater,
  Less,
  GreaterOrEqual,
  LessOrEqual,
  NotEqual,
  Or,
  And,
}

impl fmt::Display for Opcode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self{
      Opcode::Mul            => write!(f, "*"),
      Opcode::Div            => write!(f, "/"),
      Opcode::Add            => write!(f, "+"),
      Opcode::Sub            => write!(f, "-"),
      Opcode::Equal          => write!(f, "=="),
      Opcode::Greater        => write!(f, ">"),
      Opcode::Less           => write!(f, "<"),
      Opcode::GreaterOrEqual => write!(f, ">="),
      Opcode::LessOrEqual    => write!(f, "<="),       
      Opcode::NotEqual       => write!(f, "!="),
      Opcode::Or             => write!(f, "||"),  
      Opcode::And            => write!(f, "&&"), 
    }
  }
}
// Opens hole configs file of hole variable assignments
// and initializes a HashMap from hole vaiables to
// i32s.
pub fn extract_hole_cfgs (hole_cfgs_file : String) -> HashMap <String, i32> {

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
      let val = hole_entry[1]
                .to_string()
                .parse::<i32>()
                .expect("Error : hole value set to non-integer value");
      hole_cfgs_map.insert (hole_entry[0].to_string(), 
                            val);
  }
  hole_cfgs_map
}

// Generates helper code for ALU function by 
// using static variable HELPER_STRING. 

pub fn get_generated_helper_string() -> String
{
  HELPER_STRING.read().unwrap().clone()
}

fn generate_mux2_optimized (mux3_name : String,
                            opcode : i32) {
  let fn_header : String = 
      format!("fn {}(op1 : i32, op2 : i32) -> i32{{\n", 
              mux3_name);
  let fn_body : String = 
      match opcode {
        0 => String::from("  op1\n}\n"),
        _ => String::from("  op2\n}\n"),
      };
  let mux2_fn : String = format!("{}{}", 
                                 fn_header,
                                 fn_body);
  HELPER_STRING.write().unwrap().push_str(&mux2_fn);
}

fn generate_mux2 (mux2_name : String) 
{
  let fn_header : String = 
      format!("fn {}(op1 : i32, op2 : i32, ctrl : i32) -> i32{{\n", 
              mux2_name);
  let if_ret : String = String::from
      ("  if ctrl == 0 {\n    op1\n  }\n");
  let else_ret : String = String::from
      ("  else {\n  op2\n  }\n}\n");

  let mut mux2_fn : String = String::from("");
  mux2_fn.push_str (&fn_header);
  mux2_fn.push_str(&if_ret);
  mux2_fn.push_str(&else_ret);

  HELPER_STRING.write().unwrap().push_str (&mux2_fn);
}

fn generate_mux3_optimized (mux3_name : String,
                            opcode : i32) {
  let fn_header : String = 
      format!("fn {}(op1 : i32, op2 : i32, op3 : i32) -> i32{{\n", 
              mux3_name);
  let fn_body : String = 
      match opcode {
        0 => String::from("  op1\n}\n"),
        1 => String::from("  op2\n}\n"),
        _ => String::from("  op3\n}\n"),
      };
  let mux3_fn : String = format!("{}{}", 
                                 fn_header,
                                 fn_body);
  HELPER_STRING.write().unwrap().push_str(&mux3_fn);
}
fn generate_mux3 (mux3_name : String)
{
  let fn_header : String = 
      format!("fn {}(op1 : i32, op2 : i32, op3 : i32, ctrl : i32) -> i32{{\n", 
              mux3_name);
  let if_ret : String = String::from
      ("  if ctrl == 0 {\n    op1\n  }\n");
  let else_if_ret : String = String::from
      ("  else if ctrl == 1 {\n    op2\n  }\n");
  let else_ret : String = String::from
      ("  else {\n  op3\n  }\n}\n");

  let mux3_fn : String= format! ("{}{}{}{}", fn_header, 
                                 if_ret, else_if_ret, else_ret);

  HELPER_STRING.write().unwrap().push_str (&mux3_fn);
}

fn generate_rel_op_optimized (rel_op_name : String,
                              opcode : i32) {
  let fn_header : String = 
      format!("fn {}(op1 : i32, op2 : i32) -> i32{{\n", 
              rel_op_name);
  let fn_body : String = 
      match opcode {
        0 => String::from("  (op1 != op2) as i32\n}\n"),
        1 => String::from("  (op1 < op2) as i32\n}\n"),
        2 => String::from("  (op1 > op2) as i32\n}\n"),
        _ => String::from("  (op1 == op2) as i32\n}\n"),
      };
  let rel_op_fn : String = format!("{}{}", 
                                 fn_header,
                                 fn_body);
  HELPER_STRING.write().unwrap().push_str(&rel_op_fn);
}

fn generate_rel_op (rel_op_name : String)
{
  let fn_header : String = 
      format!("fn {} (op1 : i32, op2 : i32, opcode : i32) -> i32{{\n",
              rel_op_name);
  
  let if_ret : String = String::from
      ("  if opcode == 0 {\n    (op1 != op2) as i32\n  }\n");
  let elif_ret_1 : String = String::from
      ("  else if opcode == 1{\n    (op1 < op2) as i32\n  }\n");
  let elif_ret_2 : String = String::from
      ("  else if opcode == 2{\n    (op1 > op2) as i32\n  }\n");
  let else_ret : String = String::from
      ("  else{\n    (op1 == op2) as i32\n  }\n}\n");
  let mut rel_op_fn : String = String::from("");
  rel_op_fn.push_str (&fn_header);
  rel_op_fn.push_str (&if_ret);
  rel_op_fn.push_str (&elif_ret_1);
  rel_op_fn.push_str (&elif_ret_2);
  rel_op_fn.push_str (&else_ret);
  
  HELPER_STRING.write().unwrap().push_str(&rel_op_fn);
} 
fn generate_arith_op_optimized (arith_op_name : String,
                                opcode : i32) {
  let fn_header : String = 
      format!("fn {}(op1 : i32, op2 : i32) -> i32{{\n", 
              arith_op_name);
  let fn_body : String = 
      match opcode {
        0 => String::from("  op1 + op2\n}\n"),
        _ => String::from("  op1 - op2\n}\n"),
      };
  let arith_op_fn : String = format!("{}{}", 
                                     fn_header,
                                     fn_body);
  HELPER_STRING.write().unwrap().push_str(&arith_op_fn);
}


fn generate_arith_op (arith_op_name : String)
{
  let fn_header : String =
      format!("fn {} (op1 : i32, op2 : i32, opcode : i32) -> i32{{\n",
              arith_op_name);
  let if_ret : String = String::from
      ("  if opcode == 0 {\n    op1 + op2\n  }\n");
  let else_ret : String = String::from 
      ("  else {\n  op1 - op2\n  }\n}\n");
  let mut arith_op_fn : String = String::from("");
  arith_op_fn.push_str (&fn_header);
  arith_op_fn.push_str (&if_ret);
  arith_op_fn.push_str (&else_ret);
  HELPER_STRING.write().unwrap().push_str (&arith_op_fn);
}

fn generate_constant_optimized (constant_name : String,
                                opcode : i32) {
  let fn_header : String = 
      format!("fn {}() -> i32{{\n", 
              constant_name);
  let temp_constant_vec : Vec <String> = CONSTANT_VEC.read()
                                                     .unwrap()
                                                     .clone();
  let fn_body : String = 
      match opcode >= temp_constant_vec.len() as i32 &&
            temp_constant_vec.len() > 0 {
        true => format!("  {}\n}}\n",
            temp_constant_vec[temp_constant_vec.len()-1]),
        false => match temp_constant_vec.len() > 0 {
             true => format!("  {}\n}}\n", 
                 temp_constant_vec[opcode as usize]),
             false => format!("  {}\n}}\n", opcode),
         }
      };
  let constant_fn : String = format!("{}{}", 
                                     fn_header,
                                     fn_body);
  HELPER_STRING.write().unwrap().push_str(&constant_fn);
}

fn generate_constant(constant_name : String)
{
  let fn_header : String = 
       format! ("fn {} (constant : i32) -> i32 {{\n",
           constant_name);
  let temp_constant_vec : Vec <String> = CONSTANT_VEC.read()
                                                     .unwrap()
                                                     .clone();
   let mut constant_vec : Vec <i32> = Vec::new();
   for x in temp_constant_vec.iter(){
     constant_vec.push (x.parse::<i32>().unwrap());
   }
    let ret = match temp_constant_vec.len() == 0 {
        true => format!("  constant\n}}"),
        false => format!("  let constant_vec = vec!{:?};\n  constant_vec[constant as usize]\n}}",
            constant_vec),
    };
    let mut const_fn : String = String::from("");
    const_fn.push_str (&fn_header);
    const_fn.push_str (&ret);
    HELPER_STRING.write().unwrap().push_str (&const_fn);

}
fn generate_opt_optimized (opt_name : String,
                           opcode : i32) {
  let fn_header : String = 
      format!("fn {}(op : i32) -> i32{{\n", opt_name);
  let fn_body : String = 
      match opcode {
        0 => String::from("  op\n}\n"),
        _ => String::from("  0\n}\n"),
      };
  let opt_fn : String = format!("{}{}", 
       fn_header,
       fn_body);
  HELPER_STRING.write().unwrap().push_str(&opt_fn);
}


fn generate_opt(opt_name : String)
{
  let fn_header : String =
      format! ("fn {} (op : i32, enable : i32) -> i32 {{\n",
               opt_name);
  let if_ret : String = String::from
      ("  if enable != 0 {\n    0\n  }\n");
  let else_ret : String = String::from
      ("else{\n  op\n  }\n}\n");
  let mut opt_fn = String::from("");
  opt_fn.push_str (&fn_header);
  opt_fn.push_str (&if_ret);
  opt_fn.push_str (&else_ret);
  HELPER_STRING.write().unwrap().push_str (&opt_fn);
}

