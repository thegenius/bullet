# bullet
bullet is a project boilerplate generator

# install
## linux/unix/mac
1. install rust: curl https://sh.rustup.rs -sSf | sh
2. install bullet: cargo install cargo-bullet
3. check version: cargo bullet --version

## windows
1. install rust: following [rust-lang](https://www.rust-lang.org/tools/install)
2. install bullet: cargo install cargo-bullet
3. check version: cargo bullet --version

## install from source
1. install rust
2. git clone https://github.com/thegenius/bullet.git
3. cd bullet
4. cargo build --release
4. ./target/release/bullet.exe --version

# usage
cargo bullet --config=\<config_file\> --template=\<template_dir\> --out=\<out_dir\>

## config file example: build.toml
```toml
group = "example"
project = "test"
```

## project structure example: templates/structure/structure.toml
```text
project_structure = [
    ["{{ context.group }}.{{context.project }}", "hello.txt", "hello.txt.tmpl"]                                            "api_hello.proto.tmpl"],
]
```

## project template example: templates/template/hello.txt.tmpl
```text
hello {{ context.project }}
```

## generate the project
```text
cargo bullet -c build.toml -t templates -o output
```

# idea
when we develop a project, we always write many boilerplate code, config and etc.
so to save your life time.
1. create a project template.
2. config the project with yaml or json or toml.
3. generate the project


# supported templates
## java spring
./release/bullet_win_0.1.exe -c release/build_example.toml -t release/templates/spring_java -o output
