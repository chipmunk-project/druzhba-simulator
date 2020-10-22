
import argparse
import sys
import subprocess
import os

def run_dgen_unoptimized (args):
    subprocess.run(['cp',
                    'dgen/target/debug/dgen',
                    'dgen_bin'])

    with open (os.devnull, 'w') as FNULL:
        output = None
        if args[6] == '':
            output = subprocess.run(['./dgen_bin',
                 args[0],  # Program name
                 args[1],  # Stateful ALU
                 args[2],  # Stateless ALU
                 args[3],  # Pipeline depth
                 args[4],  # Pipeline width
                 args[5],  # Stateful ALUs per stage
                 '-o',
                 'prog_to_run.rs',  # Output prog_to_run
                 ], stderr=FNULL)

        else:
            output = subprocess.run(['./dgen_bin',
                 args[0],  # Program name
                 args[1],  # Stateful ALU
                 args[2],  # Stateless ALU
                 args[3],  # Pipeline depth
                 args[4],  # Pipeline width
                 args[5],  # Stateful ALUs per stage
                 '-c', 
                 args[6],  # Constant vec
                 '-o',
                 'prog_to_run.rs',  # Output prog_to_run
                 ],  stderr=FNULL)
    subprocess.run(['rm',
                    'dgen_bin'])

    output.check_returncode()
    subprocess.run(['mv',
                    'prog_to_run.rs',
                    'src'])

def run_dsim(args):

    output = None
    with open (os.devnull, 'w') as FNULL:
        output = subprocess.run(['cargo',
             'run',
             '--',
             '-g',
             args[8],
             '-t',
             args[9],
             '-i',
             args[7],
             '-s',
             args[11],
             '-p',
             args[12]], stderr=FNULL)
        output.check_returncode()

def run_dgen_optimized (args):
    subprocess.run(['cp',
             'dgen/target/debug/dgen',
             'dgen_bin'])
    output = None
    with open (os.devnull, 'w') as FNULL:
        if args[6] == '':
            output = subprocess.run(['./dgen_bin',
                 args[0],  # Program name
                 args[1],  # Stateful ALU
                 args[2],  # Stateless ALU
                 args[3],  # Pipeline depth
                 args[4],  # Pipeline width
                 args[5],  # Stateful ALUs per stage
                 '-o',
                 ' prog_to_run.rs',  # Output prog_to_run
                 '-i',
                  args[7],  # Hole configurations
                 '-O',
                 args[10],  # Optimization level
                 ], stderr=FNULL)
        else: 
            output = subprocess.run(['./dgen_bin',
                args[0],  # Program name
                args[1],  # Stateful ALU
                args[2],  # Stateless ALU
                args[3],  # Pipeline depth
                args[4],  # Pipeline width
                args[5],  # Stateful ALUs per stage
                '-c',
                args[6],  # Constant vec
                '-o',
                'prog_to_run.rs',  # Output prog_to_run
                '-i',
                 args[7],  # Hole configurations
                '-O',
                 args[10],  # Optimization level
                ], stderr=FNULL)
    subprocess.run(['rm',
        'dgen_bin'])
    output.check_returncode()
    subprocess.run(['mv',
        'prog_to_run.rs',
        'src'])


def rerun_dsim (args):
    subprocess.run(['cp',
         'target/debug/druzhba',
         'dsim_bin'])
    output = None
    with open(os.devnull, 'w') as FNULL:
        output = subprocess.run(['./dsim_bin',
             '-g',
             args[8],
             '-t',
             args[9],
             '-i',
             args[7],
             '-s',
             args[11],
             '-p',
             args[12]], stderr=FNULL)
    subprocess.run(['rm',
        'dsim_bin'])

    output.check_returncode()

def main ():
    argv = sys.argv
    parser = argparse.ArgumentParser(description='Druzhba execution')
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
            '-c',   
            '--constants',
            nargs='?', 
            type=str,
            default='',
            help='Constant vector for Chipmunk')
    parser.add_argument(
            'hole_configs',
            type=str,
            help='File path for the file containing the machine code assignments')
    parser.add_argument(
            '-g',
            '--gen',
            type=int,
            default='0',
            help='Number of PHV containers to randomly initialize by traffic generator. Rest of PHV containers initialized with 0')
    parser.add_argument(
            '-t',
            '--ticks',
            nargs='?',
            type=int,
            default='100',
            help='Number of ticks')
    parser.add_argument(
            '-O',
            '--opti',
            nargs='?',
            type=int,
            default='0',
            help='Number corresponding to optimization level (0 for unoptimized, 1 for sparse conditional constant propagation, 2 for inlining)')
    parser.add_argument(
            '-s',
            '--state',
            nargs='?',
            type=str,
            default='{}',
            help='Initial pipeline state variable values (provided in the form: \"{{state_group_0_state_0, state_group_0_state_1, ...}, {state_group_1_state_0, state_group_1_state_1, ...}, ...}\"')
    parser.add_argument(
            '-p',
            '--phv',
            nargs='?',
            type=str,
            default='',
            help='Initial PHV values in form \"{x_1, x_2, ... \"}')

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
    args.append(raw_args.constants)
    args.append(raw_args.hole_configs)
    if raw_args.gen == 0:
        args.append(str(raw_args.pipeline_width))
    else:
        args.append(str(raw_args.gen))
    args.append(str(raw_args.ticks))
    opti = raw_args.opti
    args.append(str(opti))
    args.append(raw_args.state)
    args.append(raw_args.phv)
    no_recompile = parser.parse_args().n

    if no_recompile:
        print('No recompile flag set') 
        rerun_dsim(args)
        exit(0)

    elif opti == 0:
        print('Building dgen ... ')
        subprocess.run(['./build_dgen.sh'])
        run_dgen_unoptimized(args)
        print('dgen completed')
        print('Preparing dsim for execution (this may take a few minutes) ... ')
    else:

        print('Building dgen ... ')
        subprocess.run(['./build_dgen.sh'])
        run_dgen_optimized(args)
        print('dgen completed')
        print('Preparing dsim for execution (this may take a few minutes) ... ')
    run_dsim(args)

if __name__== "__main__":
    main()
