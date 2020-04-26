[![Build status](https://ci.appveyor.com/api/projects/status/8bmj5q0xotaa9xe8/branch/master?svg=true)](https://ci.appveyor.com/project/mdw362/druzhba-public/branch/master)


# `Druzhba`

# Overview

Designing and developing compilers for programmable switches is challenging due to the restraints
of switch architecture. Druzhba attempts to mitigate this issue by providing a test platform for
these compilers. High level programs compiled to Druzhba's instruction set can be simulated, testing
the abilities for these compilers to map programs to switch hardware primitives. Druzhba enables 
simulation through 2 components: dgen and dsim. dgen generates a Rust file corresponding to the
pipeline to be simulated and dsim uses this file to model the pipeline's feedforward behavior.

# Installation

1. Install rust 

2. Clone this repo. Note: rust nightly may need to be enabled:

    ```rustup default nightly```

3. You're good to go! 

# Usage

To easily execute Druzhba use druzhba_run.py (this will execute both dgen and dsim):

    python3 druzhba_run.py -h

    usage: druzhba_run.py [-h] [-n]
                          program_name stateful_alu stateless_alu
                          pipeline_depth pipeline_width num_stateful_alus
                          constant_set hole_configs num_phvs ticks opt_level

    dsim execution

    positional arguments:
      program_name       Program spec name
      stateful_alu       Path to stateful ALU file
      stateless_alu      Path to stateless ALU file
      pipeline_depth     Depth of pipeline
      pipeline_width     Width of pipeline
      num_stateful_alus  Number of stateful ALUs per stage (number of state
                         variables in spec)
      constant_set       Constant vector
      hole_configs       File path for the file containing the machine code
                         assignments
      num_phvs           Number of PHV containers to randomly initialize by
                         traffic generator. Rest of PHV containers initialized
                         with 0
      ticks              Number of ticks
      opt_level          Number corresponding to optimization level (0 for
                         unoptimized, 1 for sparse conditional constant
                         propagation, 2 for inlining)

    optional arguments:
      -h, --help         show this help message and exit
      -n                 Set if attempting to simulate the previous configuration
                         to prevent recompiling dsim

Example:

    python3 druzhba_run.py simple example_alus/stateful_alus/raw.alu example_alus/stateless_alus/stateless_alu.alu 2 2 1 "0,1,2,3" hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt 1 100 1

More examples can be found at the bottom. Note: the -n recompile flag should be used if you have already compiled dsim previously and would like to rerun it without recompiling.
This is especially useful if machine code pairs are to be swapped for unoptimized Druzhba executions 
or if the program is to be run with a different number of ticks. 
If the recompile flag is turned
on for a new pipeline configuration (e.g. different stateful ALU or pipeline dimensions) this will not work; dgen will need to be run again
and the resulting file needs to be compiled with dsim.

To execute dgen alone (note that this is within the dgen directory):

    dgen -h

    dgen 1.0
    Code generator for Druzhba

    USAGE:
        dgen_bin [OPTIONS] <spec_name> <stateful_alu> <stateless_alu> <pipeline_depth> <pipeline_width> <num_stateful_alus> <constant_vec> [machine_code]

    ARGS:
        <spec_name>            Name of input program
        <stateful_alu>         Path to stateful ALU file
        <stateless_alu>        Path to stateless ALU file
        <pipeline_depth>       Depth of pipeline to simulate
        <pipeline_width>       Width of pipeline to simulate
        <num_stateful_alus>    Number of stateful ALUs per stage
        <constant_vec>         Constant vector for Chipmunk
        <machine_code>         Druzhba machine code (only required for optimzed code generation)

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -O, --opti <opti>             Optimization level: 0, 1, or 2
        -o, --output <output_file>    Output generated file

Example:

    cd dgen
    cargo run simple ../example_alus/stateful_alus/raw.alu ../example_alus/stateless_alus/stateless_alu.alu 2 2 1 "0,1,2,3"  -o ../src/prog_to_run.rs 


The output will be a Rust file containing the pipeline description to be simulated. To compile with dsim, rename the file to 
prog_to_run.rs and move it into the src directory.

To execute dsim:

    dsim -h

    Druzhba 1.0
    Hardware switch simulator for compiler testing

    USAGE:
        dsim_bin [OPTIONS] <containers> <ticks>

    ARGS:
        <containers>    Number of PHV containers to be initialized by traffic generator
        <ticks>         Number of ticks to execute for. A PHV enters the pipeline at every tick.

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -f, --file <file>    Path to file containing machine code pairs.


Example:

    cargo run 1 20 -f hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt 2>/dev/null


If optimizations are desired (using -O1 or -O2 in dgen), the machine code is fed into dgen instead of dsim.

The above example directs STDERR to null device due to compiler warnings from test generated dgen files. Many of these warnings 
are either unavoidable or helpful for readibilty. While some can be fixed we leave that to future work due to the tediousness 
of the task. 

# Test

Tests will ensure the druhzba pipeline is outputting
the correct packets relative to the input packets
given to the pipeline. Run dgen first to get a 
prog_to_run.rs file in src first. Otherwise the simulator
won't compile. 

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


# More Examples


Simulating marple_tcp_nmo:

    python3 druzhba_run.py marple_tcp_nmo_equivalent_1_canonicalizer_equivalent_0 example_alus/stateful_alus/pred_raw.alu example_alus/stateless_alus/stateless_alu.alu 3 2 2 "0,1,2,3" hole_configurations/marple_tcp_nmo_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt 1 20 2
Simulating blue_increase:

    python3 druzhba_run.py blue_increase_equivalent_2_canonicalizer_equivalent_0 example_alus/stateful_alus/pred_raw.alu example_alus/stateless_alus/stateless_alu_arith.alu 4 2 2 "11,21,10,12,0,3,1,2,10,2,1"  hole_configurations/blue_increase_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2_hole_cfgs.txt

Simulating snap_heavy_hitter:

    python3 druzhba_run.py snap_heavy_hitter example_alus/stateful_alus/pair.alu example_alus/stateless_alus/stateless_alu.alu 2 3  1 "0,1,2,3,999,997,1002,1000,4"  hole_configurations/snap_heavy_hitter_pair_stateless_alu_2_3_hole_cfgs.txt 1 20 2

Simulating times two:

    python3 druzhba_run.py times_two example_alus/stateful_alus/sub.alu example_alus/stateless_alus/stateless_alu_arith.alu 3 3 1 "0,1,2,3"  hole_configurations/times_two_sub_stateless_alu_arith_3_3_hole_cfgs.txt 1 100 1

Simulating test algorithm:

    python3 druzhba_run.py test example_alus/stateful_alus/if_else_raw.alu example_alus/stateless_alus/stateless_alu_arith_rel_cond.alu 4 4 2 "0,1,2,3"  hole_configurations/test_if_else_raw_stateless_alu_arith_rel_cond_4_4_hole_cfgs.txt 3 100 1

