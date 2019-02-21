# bullet
bullet is a project boilerplate generator

# idea
when we develop a project, we always write many boilerplate code, config and etc.
so to save your life time.
1. create a project template.
2. config the project with yaml or json or toml.
3. generate the project

# usage
bullet.exe --config=\<config_dir\> --template=\<template_dir\> --out=\<out_dir\>

# build install
1. rust
2. cargo
3. cargo build --release
4. ./target/release/bullet.exe --version

# supported templates
## java spring
./release/bullet_win_0.1.exe -c release/build_example.toml -t release/templates/spring_java -o output

# pre-build download
![windows-64](https://github.com/thegenius/bullet/blob/master/release/bullet_win64_0.1.exe)