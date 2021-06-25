cd /home/logcreative/下载/apache-tvm-src-v0.6.1.rc1-incubating
# cd build
# cmake ..
# make -j4
# cd ..
cd python
python3 setup.py install --user
cd ..
cd topi
cd python
python3 setup.py install --user