# Since Linux has LLVM built-in
# Thus no additional LLVM installation needed.
# Open set(USE_LLVM ON) in config.cmake to compile.

# clone
git clone --recursive https://github.com/apache/tvm tvm
cd tvm
git submodule init
git submodule update

# compile
cd build
cmake ..
make -j4

# install
cd ..
cd python
python3 setup.py install --user