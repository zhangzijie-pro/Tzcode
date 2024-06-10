## python说明


注: 函数输入参数类型为string的path，通过fs读取文件内容并进行对应的逻辑内容后返回保存文件并返回

1.用于简易的代码补全
```sh
jedi_tauri.py
```

2.Black 是一个无配置的 Python 代码格式化工具
```sh
format_code.py
```

3.Pylint 进行语法检查
```sh
lgu.py
```

对应库有：
```js
语法检测和语言分析：
    *Pylint: 用于检查Python代码中的错误，遵循编码标准。
    Pyflakes: 主要用于查找Python代码中的错误。
    *Flake8: 集成了Pyflakes、pycodestyle和Ned Batchelder的McCabe script。
    Bandit: 专注于Python代码的安全性分析。
    *Mypy: 用于静态类型检查。

代码格式化：
    Black: “代码格式化即服务”，一个无争议的Python代码格式化工具。
    YAPF (Yet Another Python Formatter): 谷歌开发的代码格式化工具。
    *autopep8: 自动将Python代码格式化为PEP 8风格。

代码补全和编辑器集成：
    *Jedi: Python静态分析工具，提供代码补全和其他智能功能。
    Pyright: 快速的Python类型检查器和语言服务器。
    Language Server Protocol (LSP): 一种与编辑器集成的协议，支持多种编程语言，包括Python。

代码风格和质量检查：
    Pycodestyle (之前称为pep8): 检查Python代码是否符合PEP 8编码风格。
    Radon: 用于计算代码的复杂度。
    Prospector: 综合了多个工具（如Pylint、pep8、Mypy等）的代码分析工具
```

对于其他语言内容的检测仅需测试 html,js/ts,rust,c
```js
JavaScript/TypeScript
    ESLint: 用于识别和报告ECMAScript/JavaScript代码中的模式。
    JSHint: 一个用于检测JavaScript代码中的错误和潜在问题的工具。
    *Prettier: 一个支持多种语言的代码格式化工具，常用于JavaScript/TypeScript。
    TypeScript: 自带的类型检查和编译器可以进行语法和类型检查。

C/C++
    Cppcheck: 一个静态分析工具，用于C/C++代码。
    Clang-Tidy: 一个基于Clang/LLVM的C++ “linter”工具。
    Uncrustify: 一个用于C、C++、C#、ObjectiveC、D、Java、Pawn和Vala代码的代码格式化工具。
    Astyle (Artistic Style): 用于格式化C、C++、C#和Java代码。

Rust
    Clippy: Rust编译器插件，提供Linting功能。
    rustfmt: Rust代码格式化工具。

HTML/CSS
    Stylelint: 一个CSS的代码风格检查工具。
    htmlhint: 用于HTML的代码检查工具。
```