
[![crates.io](https://img.shields.io/badge/crates.io-0.2.0-green.svg)](https://crates.io/crates/cargo-bullet)
[![Build Status](https://www.travis-ci.org/thegenius/bullet.svg?branch=master)](https://www.travis-ci.org/thegenius/bullet)

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
[ext]
my_content = "This is my content"
```

## project structure example: templates/structure/structure.toml
```text
project_structure = [
    {item_path="{{ context.group }}.{{context.project }}",  item_file = "hello.txt", item_tmpl="hello.txt.tmpl", item_type="static"}
]
```

## project template example: templates/template/hello.txt.tmpl
```text
hello {{ context.project }} {{ context.ext.my_content }}
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

# For Template Developer
## Template Structure
```text
root |-- structure/
     |-- |-- structure.toml 
     |-- template/
     |-- |-- hello.tmpl
     |-- |-- world.tmpl
     |-- bullet.toml
```

## Structrue File
1. project_structure is the root element
2. children element must have 4 fields: item_path, item_file, item_tmpl, item_type
3. item_path is the file path that you want to generate
4. item_file is the file name that you want to generate
5. item_tmpl is the template file located in template dir
6. item_type now support "static" and "dynamic"
### Static Structure
you can use context as the reference to bullet.toml properties  
for example:
``` text
item_path = "{{context.project}}.hello"
```
### Dynamic Structure
you can use context as the reference to bullet.toml properties,   
and you can use resource as the reference to bullet.toml's resource item
for example:
``` text
item_path = "{{context.project}}.hello" item_file = "{{resource.name_info.default_name}}"
```

## For All User
### bullet.toml example
```text
group = "example"
project = "test"

[ext]
content = "ext content"

[resources.basic_info]
name_info = {default_name = "hello"}
type_info = {}
ext = {}
fields = [
    {name_info = {default_name = "id", camel_name="id"},  type_info= {java="Long"}},
]
```
### bullet.toml root element
|property|must|type|
|----|----|----|
|group|yes|string|
|project|yes|string|
|ext|no|map\<string, string\>|
|resources|no|resource element|

### resource element
|property|must|type|
|----|----|----|
|name_info|yes|name element|
|type_info|yes|map\<string, string\>|
|ext|no|map\<string, string\>|
|fields|yes|field element|

### field element
|property|must|type|
|----|----|----|
|name_info|yes|name element|
|type_info|yes|map\<string, string\>|
|ext|no|map\<string, string\>|

### name element
|property|must|type|
|----|----|----|
|default_name|yes|string|
|snake_name|no|string|
|hyphen_name|no|string|
|upper_camel_name|no|string|
|lower_camel_name|no|string|

# Supported Templates
## Java Spring
```text
cargo bullet install --name=bullet-spring-java
cargo bullet create  --name=bullet-spring-java
cargo bullet build   --name=bullet-spring-java --output=out 
```
