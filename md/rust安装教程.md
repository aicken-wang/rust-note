# Rust 学习
### Rust安装
# 设置代理
- Linux or MacOS
- Windows添加环境变量
```bash
# linux 
vim ${HOME}/.bash.rc
# 添加到最后
export RUST_HOME=/opt/rust-toolchain # 安装前配置，设置工具链存储的路径
export CARGO_HOME=$RUST_HOME 
# 设置代理 避免被GFW，国内加速
export RUSTUP_DIST_SERVER=https://rsproxy.cn
export RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup
source ${HOME}/.bashrc
echo ${CARGO_HOME}
```

```bash

# 安装到/opt/rust目录下,linux下普通用户为username是wang的用户添加权限
# wang默认在/opt/rust没有权限，通过acl工具添加权限，也可以简单粗暴 sudo chown -R wang.wang /opt/rust
sudo apt install acl
useradd wang
# 将wang添加到root组
sudo usermod -aG root wang
# 查看wang是否在root组内
groups wang
# 为wang添加读写执行权限
setfacl -m u:wang:rwx /opt/rust
# 更多用法man setfacl 查看
# MacOS直接安装到$HOME
```
- MacOS or Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Windows下载最新版本`rustup-init.exe`即可（没有安装(Visual Studio)MSVC编译器的可选择GNU工具链版本）
  - [32 位的版本](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe) 
  - [64 位的版本](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
  - 使用MSVC工具链的请先安装[Visual Studio](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html)
- 体验Rust语法的可以前往[Rust 试验场](https://play.rust-lang.org/)

- [vs code](https://code.visualstudio.com/)
- 安装插件`rust-analyzer`，其它的按需安装即可

- `rust` 工具链介绍
  - rustup:用来升级维护Rust编译器，同时支持多个版本，可安装Rust组件
```bash
rustup update # 更新所有已安装的工具链
rustup update stable # 升级到最新稳定版
rustup install stable # 安装最新稳定版
rustup install nightly # 安装夜间版即Beta版，可体验最新特性可能不稳定
# 设置默认版本
rustup default stable # 设置全局默认使用稳定版
rustup default nightly # 切换到夜间版
rustup overide set 1.58.0 # 在当前目录下强制使用指定版本
rustup run nightly cargo build # 临时使用夜间版编译构建

# 组件管理 （添加/移除组件）
rustup component add rustfmt # 安装代码格式化工具, 统一代码风格; 在工程目录下执行: cargo fmt 
rustup component add clippy # 安装代码检查工具，可对代码进行严格检查，指出不规范的地方; 在工程目录下执行: cargo clippy
rustup component remove rustfmt # 移除组件

# 管理跨平台编译目标
rustup target add  x86_64-unknown-linux-musl # 添加Linux静态编译目标
# 查看 已安装工具链
rustup toolchain list # 查看所有已安装的版本

rustup show # 查看当前Rust工具链及组件
rustup uninstall 1.58.0 # 卸载工具链

# 交叉编译到musl环境
# rustup target add x86_64-unknown-linux-musl # 若没有安装的需要先安装Linux静态编译目标
# rustup target add x86_64-unknown-linux-gnu # 若没有安装的需要先安装Linux GNU静态编译目标
cargo build --targe=x86_64-unknown-linux-musl # 生成静态二进制文件
# MacOS下报错

# TODO: MacOS下编译前需要执行:sudo xcodebuild -license

rustup run beta cargo test # 使用beta版运行测试

# 其它
rustup --help # 查看完整命令列表

```

- **cargo**:Rust的包管理器，构建工具和依赖解决器，可以用cargo 命令创建Rust项目
```bash
# 创建一个demoProject项目
# 默认是可执行程序，--bin可以省略，若要创建库需指定--lib
cargo new --bin demoProject 
# 等效 cargo new demoProject
```
- Cargo.toml是一个工程配置文件,包含了package的基本信息
```toml
[package]
name = "demoProject"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
- src是存放源码目录
- src/main.rs是rust函数入口
```rust
fn main() {
    println!("Hello, world \n Welcom Rustaceans!");
}
```

- 编译
```bash
cargo build # 会在target/debug目录下生成可执行文件 
cargo run # 直接运行
# 其它
cargo build --help
```
