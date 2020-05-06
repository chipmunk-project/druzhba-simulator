
import argparse
import sys
import subprocess
import os

def run_dgen_unoptimized (args):
    subprocess.run(['cp',
                    'dgen/target/debug/dgen',
                    'dgen_bin'])

    with open (os.devnull, 'w') as FNULL:
      subprocess.run(['./dgen_bin',
                      args[0], # Program name
                      args[1], # Stateful ALU
                      args[2], # Stateless ALU
                      args[3], # Pipeline depth
                      args[4], # Pipeline width
                      args[5], # Stateful ALUs per stage
                      args[6], # Constant vec
                      '-o prog_to_run.rs', # Output prog_to_run
                      ], stderr=FNULL)
    subprocess.run(['rm',
                    'dgen_bin'])
    subprocess.run(['mv',
                    'prog_to_run.rs',
                    'src'])

def run_dsim(args):

    with open (os.devnull, 'w') as FNULL:
      subprocess.run(['cargo',
                      'run',
                      args[8],
                      args[9],
                      '-f',
                      args[7]], stderr=FNULL)

def run_dgen_optimized (args):
    subprocess.run(['cp',
                    'dgen/target/debug/dgen',
                    'dgen_bin'])
    with open (os.devnull, 'w') as FNULL:
      subprocess.run(['./dgen_bin',
                      args[0], # Program name
                      args[1], # Stateful ALU
                      args[2], # Stateless ALU
                      args[3], # Pipeline depth
                      args[4], # Pipeline width
                      args[5], # Stateful ALUs per stage
                      args[6], # Constant vec
                      '-o prog_to_run.rs', # Output prog_to_run
                      args[7], # Hole configurations
                      ('-O' + args[10]), # Optimization level
                      ], stderr=FNULL)
    subprocess.run(['rm',
                    'dgen_bin'])
    subprocess.run(['mv',
                    'prog_to_run.rs',
                    'src'])


def rerun_dsim (args):
    subprocess.run(['cp',
                    'target/debug/druzhba',
                    'dsim_bin'])
    with open(os.devnull, 'w') as FNULL:
      subprocess.run(['./dsim_bin',
                      args[8],
                      args[9],
                      '-f',
                      args[7]], stderr=FNULL)
    subprocess.run(['rm',
                    'dsim_bin'])

def main ():
    argv = sys.argv
    parser = argparse.ArgumentParser(description='dsim execution')
    parser.add_argument(
            'program_name', 
            type=str,
            help='Program spec name')
    parser.add_argument(
            'stateful_alu', 
            type=str,
            help='Path to stateful ALU file')
    parser.add_argument(
            'stateless_alu', 
            type=str,
            help='Path to stateless ALU file')
    parser.add_argument(
            'pipeline_depth', 
            type=int,
            help='Depth of pipeline')
    parser.add_argument(
            'pipeline_width', 
            type=int,
            help='Width of pipeline')
    parser.add_argument(
            'num_stateful_alus', 
            type=int,
            help='Number of stateful ALUs per stage (number of state variables in spec)')
    parser.add_argument(
            'constant_set',
            type=str,
            help='Constant vector')
    parser.add_argument(
            'hole_configs',
            type=str,
            help='File path for the file containing the machine code assignments')
    parser.add_argument(
            'num_phvs',
            type=int,
            help='Number of PHV containers to randomly initialize by traffic generator. Rest of PHV containers initialized with 0')
    parser.add_argument(
            'ticks',
            type=int,
            help='Number of ticks')
    parser.add_argument(
            'opt_level',
            type=int,
            help='Number corresponding to optimization level (0 for unoptimized, 1 for sparse conditional constant propagation, 2 for inlining)')
    parser.add_argument(
             '-n', 
              action='store_true', 
              help='Set if attempting to simulate the previous configuration to prevent recompiling dsim')

    raw_args = parser.parse_args(argv[1:])
    args = []
    args.append(raw_args.program_name)
    args.append(raw_args.stateful_alu)
    args.append(raw_args.stateless_alu)
    args.append(str(raw_args.pipeline_depth))
    args.append(str(raw_args.pipeline_width))
    args.append(str(raw_args.num_stateful_alus))
    args.append(raw_args.constant_set)
    args.append(raw_args.hole_configs)
    args.append(str(raw_args.num_phvs))
    args.append(str(raw_args.ticks))
    opt_level = raw_args.opt_level
    args.append(str(opt_level))
    no_recompile = parser.parse_args().n

    if no_recompile:
      print('No recompile flag set') 
      rerun_dsim(args)
      exit(0)

    elif opt_level == 0:
      subprocess.run(['./build_dgen.sh'])
      print('Preparing dsim for execution (this may take a few minutes) ... ')
      run_dgen_unoptimized(args)
    else:
      subprocess.run(['./build_dgen.sh'])
      print('Preparing dsim for execution (this may take a few minutes) ... ')
      run_dgen_optimized(args)
    run_dsim(args)

if __name__== "__main__":
    main()
