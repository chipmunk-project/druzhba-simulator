[![Build status](https://ci.appveyor.com/api/projects/status/8bmj5q0xotaa9xe8/branch/master?svg=true)](https://ci.appveyor.com/project/mdw362/druzhba-public/branch/master)



Programmable switch pipeline simulator.

Before the simulator can be executed, dgen needs to build
and run. After running dgen, a prog_to_run.rs Rust file
will be automatically generated in the Druzhba src directory 
which will allow the simulator to run.


Running dgen/dsim:

    python3 execute_simulator.py <program name> <stateful ALU> <stateless ALU> <pipeline depth> <pipieline width> <stateful ALUs per stage> <constant set> <hole configurations> <packets> <ticks> <optimization level>

Example:

    python3 execute_simulator.py simple example_alus/stateful_alus/raw.alu example_alus/stateless_alus/stateless_alu.alu 2 2 1 "0,1,2,3" hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt 1 100 1

Tests will ensure the druhzba pipeline is outputting
the correct packets relative to the input packets
given to the pipeline. Run dgen first to get a 
prog_to_run file in src first. Otherwise the simulator
won't compile. 

To run these tests:

    ./build_dgen.sh && cargo test

To run benchmarks:

    ./build_dgen.sh && cargo bench

Similarly, the dgen tests ensure that the alu grammar
is being parsed correctly and that the ast is being
generated properly. 

To run these tests:

    cd dgen
    cargo test

Note: Rust nightly may need to be enabled before LALRPOP
can work

    rustup default nightly

