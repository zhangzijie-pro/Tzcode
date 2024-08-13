# Tzcode

## ! 最近学习C++,后续更新计划： 先上传c++写的跨平台文件同步工具，然后嵌套入Tzcode中 (预计10月前继续更新Tzcode)
### FileSyncTool  -> [path](https://github.com/zhangzijie-pro/FileSyncTool.git)

## 目前仅支持编辑文件和使用Tiks命令行等基础操作

终端命令来自于[**Tiks**](https://github.com/zhangzijie-pro/Tiks.git)
命令解释说明可见于Tiks的[readme.md](./tiks_command/explain.md)

webview的框架是[**tauri**](https://tauri.app/zh-cn/v1/guides/)

## [产品样式/图片](./img/product/main.png)
## 特点

- **快速轻量**：使用Rust构建，确保性能和可靠性。
- **跨平台**：借助Tauri，编辑器可在Windows、macOS和Linux上流畅运行。
- **可扩展**：支持Python脚本，用户可以自定义和扩展编辑器的功能。
- **直观界面**：简洁且用户友好的界面设计，提升生产力。

## 使用技术

- **Rust**：用于核心功能和性能。
- **Tauri**：用于创建跨平台桌面应用程序。
- **Python**：用于脚本和扩展编辑器功能。

## 本地运行
### [环境搭建](./set_env.md)
```sh
RUN:
    cargo tauri dev
```

## 许可证

本项目使用 [MIT 许可证](LICENSE)。
