## 环境变量

如果想将其加入环境变量：

```bash
    Cargo build --release
    cp | copy  ./target/release/tiks  $HOME/.Tiks/bin
    或者下载 tiks / tiks.exe
    添加至.Tiks目录下的bin中
```

- Windows:     执行./window/setup.bat    ->  setx PATH "%PATH%;%TIKS_DIR%\bin"
- mac & Linux: 执行./mac_linux/setup.sh  ->  PATH=$PATH:/$HOME/.Tiks/bin   
