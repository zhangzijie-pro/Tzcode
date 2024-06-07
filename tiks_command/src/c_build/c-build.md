## 添加C语言方法

确保你的系统中存在C/C++环境与gcc工具链

- 向文件夹c中加入你的c代码后
- 见一下代码    [详细的了解cc](https://docs.rs/cc/1.0.92/cc/)


完成后，可在command.rs中声明方法，并在arg.rs中进行配置
详见:[rust添加方法](../commands/README.md)
[代码示例<command.rs>](../commands/command.rs)
[代码示例<arg.rs>](../commands/arg.rs)

```rust
    
    //build.rs
    // C
    fn main() {
        cc::Build::new()
            .file("foo.c")
            .file("bar.c")
            .compile("foo");
    }

    // C++
    fn main(){
        cc::Build::new()
            .cpp(true)
            .file("foo.cpp")
            .compile("foo")
    }
    
    // cargo build 后会将以上.c文件编译为libfoo.a
    // 如果你的方法中包含有
    void foo_function(void) { ... }  和  int32_t bar_function(int32_t x) { ... }


    extern "C" {
        fn foo_function();
        fn bar_function(x: i32) -> i32;
    }

    pub fn call() {
        unsafe {
            foo_function();
            bar_function(42);
        }
    }

    fn main() {
        call();
    }
```