#!/usr/bin/env bash
root_dir=$(cd "$(dirname "$0")"; cd ..; pwd)
sh $root_dir/bin/build.sh

mkdir -p $root_dir/src/cbin

cd $root_dir/src
echo "gen cdylib rust_interop begin..."
# 通过动态链接的方式，生成main二进制文件
# -l参数后面是xxx名字，在Cargo.toml中 name = "rust_interop" # 链接库名字
gcc -o $root_dir/src/cbin/main $root_dir/main.c -I./src -L../../target/debug -lrust_interop
echo "gen cdylib rust_interop success"
echo "run $root_dir/src/cbin/main begin..."
$root_dir/src/cbin/main
