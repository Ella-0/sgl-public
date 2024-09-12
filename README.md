# SGL
Simple Graphics Language


## Example


```sgl

fn main(env: Env) -> Result<(), u32> {
    let spirv = shader {
        fn vert() {
            // ..v vertex shader stage
        }
    }
}
```


## In Progress
 - [ ] Add type pass to fill in a check types in the AST

## Steps

 - [ ] Unit test and rustdoc throughout

 - [ ] Simple syntax
 - [ ] define tokens
 - [ ] create finite automita for tokens
 - [ ] implement lexer around finite automita
 - [ ] work out rough grammars for the langauge
 - [ ] create a simple parser from these grammars
 - [ ] create a basic llvm code genorater of the ast
 - [ ] add a semantic stage for type checking
 - [ ] add shader block and SPIR-V backend.


### Aynalysis
In the computer graphics industry we have two main places we run code, on the GPU and on the CPU and we have two different
programming lanaguages in seperate source files for code to be run on the GPU (shader code) and code to be run on the CPU.

#### Objectives

 - To create a programming langauge that solves this by unifying shader source code and CPU source code
   by having a `shader` keyword that defines a block of code to be compiled to SPIR-V (the format used by
   open graphics libraries such as Vulkan and modern OpenGL).

 - In the current system, one has build scripts that compile shader code (usually hlsl or glsl) to SPIR-V files
   that are included into the binary usually at runtime. hlsl and glsl have different syntaxes from the host langauge
   (usually C, C++ or Rust) which can make it confusing for the programmer.

 - I will not aim to provide semantic verification of calls to OpenGL or Vulkan when loading shaders. It will still be possibly
   to have the wrong input descriptions setup with these libraries and thus the code wont work.
