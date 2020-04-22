[![Build status](https://ci.appveyor.com/api/projects/status/8bmj5q0xotaa9xe8/branch/master?svg=true)](https://ci.appveyor.com/project/mdw362/druzhba-public/branch/master)


# `Druzhba`

# Overview

Designing and developing compilers for programmable switches is challenging due to the restraints
of switch architecture. Druzhba attempts to mitigate this issue by providing a test platform for
these compilers. High level programs compiled to Druzhba's instruction set can be simulated, testing
the abilities for these compilers to map programs to switch hardware primitives.

# Installation

1. Install rust 

2. Clone this repo. Note: rust nightly may need to be enabled:

    rustup default nightly

3. You're good to go! 

# Usage

Running Druzhba:

    python3 execute_simulator.py <program name> <stateful ALU> <stateless ALU> <pipeline depth> <pipeline width> <stateful ALUs per stage> <constant set> <machine code pairs> <Number of PHV containers to initialize> <ticks> <optimization level>

Example:

    python3 execute_simulator.py simple example_alus/stateful_alus/raw.alu example_alus/stateless_alus/stateless_alu.alu 2 2 1 "0,1,2,3" hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt 1 100 1

Tests will ensure the druhzba pipeline is outputting
the correct packets relative to the input packets
given to the pipeline. Run dgen first to get a 
prog_to_run file in src first. Otherwise the simulator
won't compile. 

# Test

To run tests:

    ./build_dgen.sh && cargo test

To run benchmarks:

    ./build_dgen.sh && cargo bench

Similarly, the dgen tests ensure that the alu grammar
is being parsed correctly and that the ast is being
generated properly. 

To run these tests:

    cd dgen
    cargo test


