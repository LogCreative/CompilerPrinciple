# install uftrace
# git clone https://github.com/namhyung/uftrace.git

# compile
rustup install nightly
rustup default nightly
rustc -Z instrument-mcount -g -A dead_code src/main.rs

# uftrace
uftrace record -t 1ms ./main input1.txt
uftrace replay > replay.txt
uftrace graph
uftrace dump --flame-graph > flame.txt

# flame graph
# git clone https://github.com/brendangregg/FlameGraph
./flamegraph.pl --width 2000 flame.txt > flame.svg