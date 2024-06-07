@echo off
setlocal

@rem 获取用户主目录路径
set "HOME_DIR=%USERPROFILE%"

@rem 设置 .Tiks 文件夹路径
set "TIKS_DIR=%HOME_DIR%\.Tiks"

@rem 设置目标文件夹路径
set "BIN_DIR=%TIKS_DIR%\bin"

@rem 如果 .Tiks 文件夹不存在，则创建它
if not exist "%TIKS_DIR%" mkdir "%TIKS_DIR%"

@rem 如果 bin 文件夹不存在，则创建它
if not exist "%BIN_DIR%" mkdir "%BIN_DIR%"

endlocal