[![Build status](https://ci.appveyor.com/api/projects/status/1j9inyn5lbxitsj7?svg=true)](https://ci.appveyor.com/project/michaeldwong/druzhba-simulator)

 
# `Druzhba`

# Overview

Druzhba is a hardware switch simulator for testing compilers targeting programmable switches.
High level programs compiled to Druzhba's instruction set can be simulated, testing
the abilities for these compilers to map programs to switch hardware primitives. Druzhba enables 
simulation through 2 components: ```dgen``` and ```dsim```. ```dgen``` generates a Rust file corresponding to the
pipeline to be simulated and ```dsim``` uses this file to model the pipeline's feedforward behavior.

# Installation

1. Install ```rust```. For Unix-like OS run ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```. If further help is needed, please see https://www.rust-lang.org/tools/install.

2. Enable ```rust nightly```:

    ```rustup default nightly```

3. Clone this repo. 

4. You're good to go! 

Simple example:

    python3 druzhba_run.py simple example_alus/stateful_alus/raw.alu example_alus/stateless_alus/stateless_alu.alu 2 2 1  hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt 

# Usage

This section describes Druzhba usage using the wrapper script.

To easily execute Druzhba use druzhba_run.py (this will execute both ```dgen``` and ```dsim```):

    python3 druzhba_run.py  -h
    usage: druzhba_run.py [-h] [-c [CONSTANTS]] [-g GEN] [-t [TICKS]] [-O [OPTI]]
                          [-s [STATE]] [-p [PHV]] [-n]
                          program_name stateful_alu stateless_alu pipeline_depth
                          pipeline_width num_stateful_alus hole_configs

    Druzhba execution

    positional arguments:
      program_name          Program spec name
      stateful_alu          Path to stateful ALU file
      stateless_alu         Path to stateless ALU file
      pipeline_depth        Depth of pipeline
      pipeline_width        Width of pipeline
      num_stateful_alus     Number of stateful ALUs per stage (number of state
                            variables in spec)
      hole_configs          File path for the file containing the machine code
                            assignments

    optional arguments:
      -h, --help            show this help message and exit
      -c [CONSTANTS], --constants [CONSTANTS]
                            Constant vector for Chipmunk
      -g GEN, --gen GEN     Number of PHV containers to randomly initialize by
                            traffic generator. Rest of PHV containers initialized
                            with 0
      -t [TICKS], --ticks [TICKS]
                            Number of ticks
      -O [OPTI], --opti [OPTI]
                            Number corresponding to optimization level (0 for
                            unoptimized, 1 for sparse conditional constant
                            propagation, 2 for inlining)
      -s [STATE], --state [STATE]
                            Initial pipeline state variable values (provided in
                            the form: "{{state_group_0_state_0,
                            state_group_0_state_1, ...}, {state_group_1_state_0,
                            state_group_1_state_1, ...}, ...}"
      -p [PHV], --phv [PHV]
                            Initial PHV values in form "{x_1, x_2, ... "}
      -n                    Set if attempting to simulate the previous
                            configuration to prevent recompiling dsim


Example:

    python3 druzhba_run.py simple example_alus/stateful_alus/raw.alu example_alus/stateless_alus/stateless_alu.alu 2 2 1  hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt -c "0,1,2,3" -g 1 -s "{{28}}"


More examples can be found at the bottom. Note: the ```-n``` recompile flag should be used if you have already compiled ```dsim``` previously and would like to rerun it without recompiling.
This is especially useful if machine code pairs are to be swapped for unoptimized Druzhba executions 
or if the program is to be run with a different number of ticks. 
If the recompile flag is turned
on for a new pipeline configuration (e.g., different stateful ALU or pipeline dimensions) this will not work; ```dgen``` will need to be run again
and the resulting file needs to be compiled with ```dsim```. 

Further information about the options can be found in the next section.

# Advanced Usage

This section discusses the individual usage of both dgen and dsim together that the wrapper script takes care of.

To execute ```dgen``` alone (note that this is within the ```dgen``` directory):

    dgen -h

    dgen 1.0
    Code generator for Druzhba

    USAGE:
            dgen [FLAGS] [OPTIONS] <spec_name> <stateful_alu> <stateless_alu> <pipeline_depth> <pipeline_width> <num_stateful_alus>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -c, --constants <constant_vec>    Constant vector for Chipmunk
        -i, --input <input_file>          Druzhba machine code (only required for optimzed code generation)
        -O, --opti <opti>                 Optimization level: 0, 1, or 2
        -o, --output <output_file>        Output generated file

    ARGS:
        spec_name            Name of input program
        stateful_alu         Path to stateful ALU file
        stateless_alu        Path to stateless ALU file
        pipeline_depth       Depth of pipeline to simulate
        pipeline_width       Width of pipeline to simulate
        num_stateful_alus    Number of stateful ALUs per stage

Example:

    cd dgen
    cargo run simple ../example_alus/stateful_alus/raw.alu ../example_alus/stateless_alus/stateless_alu.alu 2 2 1 -c 0,1,2,3 -i ../hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt -O 2 
    cd ..

```dgen``` reqeires a hardware specification to to generate the pipeline description. 
spec_name is the name of the program to be run (this is the name used in the machine code value names). 
stateful_alu and stateless_alu are the .alu files. 
pipeline_depth is the number of pipeline stages and pipeline_width is the number of stateless ALUs per stage. num_stateful_alus is the number of stateful ALUs per stage.


The output of ```dgen``` will be a Rust file containing the pipeline description to be simulated. Use the ```-o``` option to specify the output file name and location. 
To compile with ```dsim```, provide the argument ```-o ../src/prog_to_run.rs```. If this is not provided, the output file must be renamed 
to prog_to_run.rs and move it into ```dsim's``` src directory. The ```-c``` argument is specific to the Chipmunk compiler; when using ```-c```, 
the immediate operand machine code values are instead treated as indices into the vector provided. ```-i``` is the file that the machine
code values reside in. ```-O``` is for optimizations; ```-O 1``` uses sprase conditional constant propagation and ```-O 2``` uses sparse
conditional constant propagation + function inlining.

To execute ```dsim```:

    dsim -h

    dsim 1.0
    Hardware switch simulator for compiler testing

    USAGE:
            dsim [FLAGS] [OPTIONS]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -s, --state <init_state_vector>    Initial value of state variables
        -i, --input <input_file>    Path to file containing machine code pairs.
        -g, --gen <num_phv_cons>    Number of PHV containers to be initialized by traffic generator
        -p, --phv <phv_initialization>     Initial PHV values in form "{x_1, x_2, ... "}
        -t, --ticks <ticks>         Number of ticks to execute for. A PHV enters the pipeline at every tick.


Example:

    cargo run -- -g 1 -t 20 -s "{{2}}" 2>/dev/null


Machine code file can be given to ```dsim``` instead of ```dgen``` if more flexibility is desired by not having to rerun ```dgen``` and recompile ```dsim``` but optimizations would not be able to be used. 

All of ```dgen's``` arguments are optional. Note that leaving out the ```-g``` option can lead to unexpected behavior depending on how the machine code was generated.
For instance, many of Chipmunk's generated programs require a specific value for ```-g``` for the intended behavior to be performed. Also note that ```dsim``` will fail to execute if no machine code pairs are provided to both ```dgen``` and ```dsim```.

The ```-s``` option is a 2D-vector of the initial state variables to be used; the format is ```"{ {s_1, s_2, ...}, {s_1, s_2, ...}, ... }"```. The ```-i``` option is the file that the machine code resides in; this is required when machine code values aren't provided to ```dgen```.
The ```-g``` option tells the traffic generator how many PHV containers to be randomly generated; the remaining PHV containers are initialized to 0. 
If this option is not used, the traffic generator will initialize all PHV containers by default with random values. The ```-p``` option is used to initialize PHV container values; the format is ```"{p_1, p_2, ... }"```.
At the moment, if this option is selected, all PHVs entering the pipeline will have the same values and aren't randomly initialized. ```-t``` denotes the number of ticks; at every tick, a new PHV is generated and enters the pipeline.

When analyzing state variables, note that each state_group corresponds to the storage within a single stateful ALU in a stage. There should be as many state groups as there are salu_config machine code values set and there should be as many state variables per state group vector as there are state variable operands the stateful ALU uses. For instance, although the example uses 1 stateful ALU per stage for 2 stages, only 1 stateful ALU is set using salu_config, thus there is 1 state group specified. And raw.alu uses only 1 state variable so the state group size is 1 element. 

The above example directs STDERR to null device due to compiler warnings from test generated ```dgen``` files. Many of these warnings are from the generated files from ```dgen``` for the units and are mostly unavoidable or helpful for readibilty. While some can be fixed we leave that for future work. 

# Test

Tests will ensure the druhzba pipeline is outputting
the correct packets relative to the input packets
given to the pipeline. Run ```dgen``` first to get a 
prog_to_run.rs file in src first. Otherwise the simulator
won't compile. All tests were previously run on Ubuntu 18.04.

To run tests:

    ./build_dgen.sh && cargo test

The benchmarks mentioned in our [paper] (https://michaeldwong.github.io/papers/druzhba_conext20.pdf) for measuring the simulation performance can also be run:

    ./build_dgen.sh && cargo bench

Similarly, the ```dgen``` tests ensure that the ALU grammar
is being parsed correctly and that the ast is being
generated properly. 

To run these tests:

    cd dgen
    cargo test


# More Examples

In this section, a few examples are provided for running different programs with varying options. 
simple is a small program used primarily for testing that is not in the paper. 
blue_increase, marple_new_flow, marple_tcp_nmo, snap_heavy_hitter, conga, flowlets, and RCP are 7 of the 12 programs
found in Table 1 of our paper.


simple:

    python3 druzhba_run.py simple example_alus/stateful_alus/raw.alu example_alus/stateless_alus/stateless_alu.alu 2 2 1  hole_configurations/simple_raw_stateless_alu_2_2_hole_cfgs.txt -c "0,1,2,3" -g 1 -t 30 -s "{{0}}"


blue_increase:

    python3 druzhba_run.py blue_increase_equivalent_2_canonicalizer_equivalent_0 example_alus/stateful_alus/pred_raw.alu example_alus/stateless_alus/stateless_alu_arith.alu 4 2 2  hole_configurations/blue_increase_equivalent_2_canonicalizer_equivalent_0_pred_raw_stateless_alu_arith_4_2_hole_cfgs.txt -g 2 -t 10 -O2 -c "11,21,10,12,0,3,1,2,10,2,1"

marple_new_flow:

    python3 druzhba_run.py marple_new_flow_equivalent_1_canonicalizer_equivalent_0 example_alus/stateful_alus/pred_raw.alu example_alus/stateless_alus/stateless_alu.alu 2 2 1 hole_configurations/marple_new_flow_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_2_2_hole_cfgs.txt -t 10 -s "{{0}}" -p "{10}" -c "0,1,2,3"

marple_tcp_nmo:

    python3 druzhba_run.py marple_tcp_nmo_equivalent_1_canonicalizer_equivalent_0 example_alus/stateful_alus/pred_raw.alu example_alus/stateless_alus/stateless_alu.alu 3 2 2  hole_configurations/marple_tcp_nmo_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_2_hole_cfgs.txt -g 1 -t 20 -O2


snap_heavy_hitter:

    python3 druzhba_run.py snap_heavy_hitter example_alus/stateful_alus/pair.alu example_alus/stateless_alus/stateless_alu.alu 2 3 1 hole_configurations/snap_heavy_hitter_pair_stateless_alu_2_3_hole_cfgs.txt -g 1 -c "0,1,2,3,999,997,1002,1000,4" -O1 -s "{{0,10}}"

conga:

    python3 druzhba_run.py conga_equivalent_1_canonicalizer_equivalent_1 example_alus/stateful_alus/pair.alu example_alus/stateless_alus/stateless_alu.alu  1 5 1 hole_configurations/conga_equivalent_1_canonicalizer_equivalent_1_pair_stateless_alu_1_5_hole_cfgs.txt -g 5 -c "0,1,2,3" -t 20 -O1

flowlets:

    python3 druzhba_run.py flowlets_equivalent_1_canonicalizer_equivalent_0 example_alus/stateful_alus/pred_raw.alu example_alus/stateless_alus/stateless_alu.alu 4 5 2 hole_configurations/flowlets_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_4_5_hole_cfgs.txt -g 3 -O1

RCP:

    python3 druzhba_run.py rcp_equivalent_1_canonicalizer_equivalent_0 example_alus/stateful_alus/pred_raw.alu  example_alus/stateless_alus/stateless_alu.alu 3 3 3 hole_configurations/rcp_equivalent_1_canonicalizer_equivalent_0_pred_raw_stateless_alu_3_3_hole_cfgs.txt -c "0,1,2,3,30,31" -g 2 


