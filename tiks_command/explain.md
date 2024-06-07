## Tiks - 简易 Linux 终端命令行工具

欢迎使用 Tiks，一个简易的 Linux 终端命令行工具，类似于常见的 Linux 终端环境，提供了一系列常用的命令和功能。

## [性能检测结果](./hyperfine.md)

## 功能特点

- 提供常用的 Linux 命令，如 `pwd`, `ls`, `cd`, `rm`, `touch`, `cat`, `python` 等。
- 支持安装和更新软件包，通过 `apt` 命令进行操作。
- 提供历史命令查看功能，通过 `history` 命令进行查看。
- 支持文件和目录的重命名，通过 `rn` 命令进行操作。
- 支持文件和目录的移动，通过 `mv` 命令进行操作。
- 提供文件压缩和解压功能，通过 `tar` 命令进行操作。
- 支持退出当前进程，通过 `exit` 命令进行操作。
- 支持&用于优先级执行，&&顺序执行,|管道符与 >重定向输出

## 使用方法

Tiks 提供了类似于常见 Linux 终端的使用方法，下面是一些常用命令的示例：
详见[命令](./src/commands/command.rs)

```sh
Usage: <command> [options] [arg]

Commands:
    pwd         查看当前目录
    ls          查看当前目录下的所有文件
    cd          切换目录
    rm          删除文件或目录
    touch       创建新文件
    cat         查看文件内容
    python      运行 Python 代码
    html        打开 HTML 文件
    apt -i      安装软件包
    apt -update 更新软件包版本
    history     查看历史命令
    rn          重命名文件或目录
    mv          移动文件或目录
    tar -zxvf:  解压缩文件
    tar -xvf:   压缩文件
    exit        退出当前进程
    ......
```

# Rust 安装工具

如果你还未安装 Rust 编程语言的开发工具链，你可以在 [Rust 官方网站](https://www.rust-lang.org/tools/install) 上找到安装说明和工具链下载链接。

点击下面的链接即可前往 Rust 官方网站下载页面：

[点击此处下载 Rust 工具链](https://www.rust-lang.org/tools/install)

## 添加方法函数

如果想向其中添加其余函数:

- C\C++: [详见添加方法](./src/c_build/README.md)
- rust: [详间rust添加方法](./src/commands/README.md)

## 环境变量
[点击此查看](./src/env/README.md)