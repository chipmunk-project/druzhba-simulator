#!/bin/bash

if [[ $# -ne 11 ]] 
then
  echo "Usage: <Program name> <Stateful ALU> <Stateless ALU> <Pipeline depth> <Pipeline width> <Stateful ALUs per stage> <Constant vector> <Machine code> <PHV containers to initialize> <Ticks> <Optimization level>"
  exit 1
fi

run_dgen_unoptimized () {
  echo "dgen unoptimized"
  chmod +x dgen_bin
  ./dgen_bin $1 $2 $3 $4 $5 $6 $7 -o "src/prog_to_run.rs"
  rm dgen_bin
}

run_dgen_optimized () {

  echo "dgen optimized"
  chmod +x dgen_bin
  ./dgen_bin $1 $2 $3 $4 $5 $6 $7 $8 -o "src/prog_to_run.rs" -O $9
  rm dgen_bin

}

run_dsim_unoptimized () {
  cargo run $1 $2 --file $3
}

run_dsim_optimized () {
  cargo run $1 $2 
}

cd dgen 
cargo build
cd ..
rm src/prog_to_run.rs
cp dgen/target/debug/dgen dgen_bin
if [[ ${11} -eq 0 ]] 
then
  echo "Unoptimized run"
  run_dgen_unoptimized $1 $2 $3 $4 $5 $6 $7
  run_dsim_unoptimized $9 ${10} $8
else
  echo "Optimized run"
  run_dgen_optimized $1 $2 $3 $4 $5 $6 $7 $8 ${11}
  run_dsim_optimized $9 ${10}
fi

