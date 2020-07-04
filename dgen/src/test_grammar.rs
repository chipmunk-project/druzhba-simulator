use std::fs;
use rust_code_generator;
use alugrammar;

#[test]
pub fn test_expr() {
  assert!(alugrammar::ExprParser::new().parse("((((C()))))").is_ok());
  assert!(alugrammar::ExprParser::new().parse("((Opt(C()))))").is_err());
  assert!(alugrammar::ExprParser::new().parse("2+2==3").is_ok());

  assert!(alugrammar::ExprParser::new().parse("thisIsAVar").is_ok());
  assert!(alugrammar::ExprParser::new().parse("2 +20>=4*2").is_ok());
  assert!(alugrammar::ExprParser::new().parse("Mux3 (arith_op (2,4), 12, Opt(10<2) )").is_ok());

  assert!(alugrammar::ExprParser::new().parse("pkt != 0 ? pkt_1 : pkt_2").is_ok());

}
#[test]
pub fn test_stmt() {
  assert!(alugrammar::StmtParser::new().parse("return Mux3 (Opt(5),12-5*2, 10);").is_ok());

  assert!(alugrammar::StmtParser::new().parse("return (pkt_0 != 0 || immediate_operand != 0);").is_ok());
  assert!(alugrammar::StmtParser::new().parse("if (2 == 2){ return 5; }").is_ok());
  assert! (alugrammar::StmtParser::new().parse(
          " if (Opt (3)) {
                return 1;
            } elif (2 == 2) {
                return 0;
            } else {
                return 4;
            }" ).is_ok());

  assert! (alugrammar::StmtParser::new().parse(
          " if (Opt (pkt_1)) {
                return 1;
            } elif (Opt(pkt_0)) {
                return 0;
            }elif (rel_op(pkt_0, pkt_1)){
                return 2;   
            }
            elif (Mux3(pkt_0, pkt_1, C()) < arith_op (state_0,pkt_1)){
                return 10;
            } else {
                return 2;   
            }").is_ok());

  assert! (alugrammar::StmtParser::new().parse("x = 3;").is_ok());

  assert! (alugrammar::StmtParser::new().parse("int old_state_0 = state_0;").is_ok());
  assert!(alugrammar::StmtParser::new().parse("return Mux2(old_state_0, state_0);").is_ok());
  assert! (alugrammar::StmtParser::new().parse("state_0 = 9;").is_ok());

}
#[test]
pub fn test_header ()
{
  assert! (alugrammar::HeaderParser::new().parse(
          " type : stateless
            state variables : {}
            hole variables : {opcode, immediate_operand}
            packet fields : {pkt_0, pkt_1}"
          ).is_ok());
  assert! (alugrammar::HeaderParser::new().parse(
          " type : state
            state variables : {state_0}
            hole variables : {opcode}
            packet fields : {pkt_0}"
          ).is_err()); 
  assert! (alugrammar::HeaderParser::new().parse(
          " type : stateful
            state variables : {state_0, state_1}
            hole variables : {}
            packet fields : {pkt_0}"
          ).is_ok());
  assert! (alugrammar::HeaderParser::new().parse(
          " type : stateful
            statee variables : {state_0}
            hole variables : {opcode}
            packet fields : {pkt_0}"
          ).is_err()); 
}
#[test]
pub fn test_alugrammar ()
{
  assert! (alugrammar::AluParser::new().parse(
          " type : stateless
            state variables : {}
            hole variables : {opcode, immediate_operand}
            packet fields : {pkt_0, pkt_1}

            if (opcode==0) {
                return pkt_1;
            } elif (opcode==1) {
                return pkt_0;
            } elif (opcode==2){
                return pkt_1 +immediate_operand;   
            }
            elif (opcode==3){
                return pkt_0 + immediate_operand;
            } else {
                return pkt_1;   
            }
            ").is_ok());
}
#[test]
pub fn test_optheader ()
{
    
  assert! (alugrammar::AluParser::new().parse(
          " name : times_two_if_else_raw_3_3
            hole configs : ../hole_configurations/hole_cfgs.txt
            pipeline stage : 2
            alu : 0
            opt level : 0
            [4,5,12,502]
            type : stateful
            state variables : {state_0}
            hole variables : {}
            packet fields : {pkt_0}

            state_0 = 0;
            
            ").is_ok());
            

  assert! (alugrammar::AluParser::new().parse(
          " name : times_two_if_else_raw_3_3
            hole configs :
            pipeline stage : 2
            alu : 0
            opt level : 1
            [0,1,2,3]
            type : stateless
            state variables : {}
            hole variables : {opcode, immediate_operand}
            packet fields : {pkt_0, pkt_1}

            if (opcode==0) {
                return pkt_1;
            } elif (opcode==1) {
                return pkt_0;
            } elif (opcode==2){
                return pkt_1 +immediate_operand;   
            }
            elif (opcode==3){
                return pkt_0 + immediate_operand;
            } else {
                return pkt_1;   
            }
            ").is_ok());
}

#[test]
pub fn test_comment()
{

  assert! (alugrammar::CommentParser::new().parse(
          "// This is a comment").is_ok());
  assert! (alugrammar::CommentParser::new().parse(
          "// This is a number 15").is_ok());

}

#[test]
pub fn test_stateless_alu ()
{

  let alu = fs::read_to_string("../example_alus/stateless_alus/stateless_alu.alu")
    .expect("Something went wrong reading the file");

  assert!(alugrammar::AluParser::new().parse(&alu).is_ok());
  // _result not used
  let _result : Box <rust_code_generator::Alu> = match alugrammar::AluParser::new().parse(&alu){
    Ok (s) => s,
    _      => panic! ("Parsing stateless ALU failed"),
  };
}
#[test]
pub fn test_stateless_alu_arith ()
{
  let alu = fs::read_to_string("../example_alus/stateless_alus/stateless_alu_arith.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());
}
#[test]
pub fn test_stateless_alu_arith_rel ()
{

   let alu = fs::read_to_string("../example_alus/stateless_alus/stateless_alu_arith_rel.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());
}
#[test]
pub fn test_stateless_alu_arith_rel_cond ()
{
 let alu = fs::read_to_string("../example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());
}
#[test]
pub fn test_stateless_alu_arith_rel_cond_bool ()
{

  let alu = fs::read_to_string("../example_alus/stateless_alus/stateless_alu_arith_rel_cond_bool.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());

}
#[test]
pub fn test_raw ()
{
  let alu = fs::read_to_string("../example_alus/stateful_alus/raw.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());
}
#[test]
pub fn test_sub ()
{
  let alu = fs::read_to_string("../example_alus/stateful_alus/sub.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());
}
#[test]
pub fn test_if_else_raw ()
{
   let alu = fs::read_to_string("../example_alus/stateful_alus/if_else_raw.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());
}
#[test]
pub fn test_nested_ifs ()
{
 let alu = fs::read_to_string("../example_alus/stateful_alus/nested_ifs.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());
}
#[test]
pub fn pred_raw ()
{
  let alu = fs::read_to_string("../example_alus/stateful_alus/pred_raw.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());
}
#[test] 
pub fn pair ()
{
  let alu = fs::read_to_string("../example_alus/stateful_alus/pair.alu")
    .expect("Something went wrong reading the file");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());

}
// Tests that all of the nodes in the rust_code_generator resulting 
// from the given spec match the expected nodes
#[test]
pub fn test_ast ()
{

  let alu = String::from(
          " type : stateless
            state variables : {}
            hole variables : {opcode, immediate_operand}
            packet fields : {pkt_0, pkt_1}

            if (opcode==0) {
                return pkt_1;
            } elif (opcode==1) {
                return pkt_0;
            } else {
                return pkt_1;   
            }
            ");
  assert! (alugrammar::AluParser::new().parse(&alu).is_ok());
  // Used in pattern match but unused otherwise
  let _result : Box <rust_code_generator::Alu> = 
      match alugrammar::AluParser::new().parse(&alu){
    Ok (s) => s,
    _      => panic! ("Parsing stateless ALU failed"),
  };

  // Asserts if condition is ID == NUM
  let check_comparison = | e1 : &rust_code_generator::Expr,
                           op : &rust_code_generator::Opcode, 
                           e2 : &rust_code_generator::Expr | -> bool {
    (match e1 {
        rust_code_generator::Expr::Var (_) => true,
        _       => false,
      }) &&
      (match op {
        rust_code_generator::Opcode::Equal => true,
        _                  => false,
      }) &&
      (match e2 {
        rust_code_generator::Expr::Num (_) => true,
        _                  => false,
      })
  };

  assert! 
  (match *_result {
    rust_code_generator::Alu::Program (opt_header, header, stmts) => {
      (match opt_header {
        // There is no OptHeader in this spec
        Some (_) => false,
        _        => true,
      }) &&
      (match *header {
        rust_code_generator::Header::InputData (state, 
                                state_vec, 
                                hole_vec, 
                                container_vec) => 
        {
            // Ensure the header has the expected
            // values
            state == "stateless" && 
            state_vec.len() == 0 &&
            hole_vec.len() == 2 &&
            hole_vec[0] == "opcode" &&
            hole_vec[1] == "immediate_operand" &&
            container_vec.len() == 2 &&
            container_vec[0] == "pkt_0" &&
            container_vec[1] == "pkt_1"
        },
      }) &&
      ( match &*stmts[0] {
        rust_code_generator::Stmt::If (expr_if, 
                       stmt_if, 
                       stmt_elif, 
                       stmt_else)  => 
        {
          (match &**expr_if {
            rust_code_generator::Expr::Op (e1, op, e2) => 
                check_comparison (&*e1, &op, &*e2),
            _                          => false,
          // Check if return statement and verifies that
          // there's only 1 statement
          }) && stmt_if.len() == 1 &&
          (match &*stmt_if[0] {
            rust_code_generator::Stmt::Return (expr) => {
              match &**expr {
                rust_code_generator::Expr::Var (s) => 
                    s == "pkt_1",
                _                                      => false,
              }
            },
            _                        => false,
          }) && stmt_elif.len() == 1 &&
          // Checks if the vector of elif statements
          // is equal to 1
          stmt_elif[0].1.len() == 1 &&
          (match &*stmt_elif[0].0 {
            rust_code_generator::Expr::Op (e1, op, e2) => 
                check_comparison (&*e1, &op, &*e2),
            _                          => false,
          }) &&
          // Check elif return statement
          (match &*stmt_elif[0].1[0] {
            rust_code_generator::Stmt::Return (expr) => {
              match &**expr {
                rust_code_generator::Expr::Var (s) => 
                    s == "pkt_0",
                _                                      => false,
              }
            },
            _                        => false,
          }) &&
          // Check else return statement
          (match stmt_else {
            Some (stmts) => {
              stmts.len() == 1 &&
              (match &*stmts[0] {
                rust_code_generator::Stmt::Return (expr) => {
                  match &**expr {
                    rust_code_generator::Expr::Var (s) => 
                        s == "pkt_1",
                    _                                      => false,
                  }
                },
                _                        => false,
              })
            },
            _           => false
          })
        },
        _   => false,
      }) 
    },
  });
}

