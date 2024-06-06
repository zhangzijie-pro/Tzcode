fn main(){
    // file: ./src/c_build/c/your.c
    // compile: your function name
    // if your function is cpp, you can {.cpp(true)}
    cc::Build::new()
        .file("./src/c_build/c/example.c")
        .compile("hello");
}