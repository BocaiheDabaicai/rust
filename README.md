## Rust

---

#### 安装

- Windows环境
  
  - 配置国内镜像【在系统环境变量里配置】，参考清华镜像源
    
    ```context
    变量名：RUSTUP_DIST_SERVER
    变量值：https://mirrors.tuna.tsinghua.edu.cn/rustup
    
    变量名：RUSTUP_UPDATE_ROOT
    变量值：https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup
    ```
  
  - 安装visiual studio installer，配置c++ 桌面开发集成，全选默认安装，大概安装11GB的工具文件
  
  - 下载 rust 安装器，安装rust即可

- 额外
  
  - 配置项目资源镜像【在`config.toml`文件中配置，没有该文件就添加】，可以选择配置**索引版本**和**稀疏索引版本**。【稀疏索引版本可以按需拉取资源，速度更快，支持1.68以上版本；传统索引版本拉取整个版本资源，速度很慢】`参考清华镜像源`
    
    - 配置的位置：`%USERPROFILE%\.cargo\config.toml`，不是项目文件
    
    ```toml
    [source.crates-io]
    replace-with = 'mirror'
    
    [source.mirror]
    registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
    
    [registries.mirror]
    index = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"
    ```

第三项配置需要移除，此为自定义注册配置，与下载依赖无关，会干扰资源下载。【移除，会影响`cargo search`、`cargo info` 等命令】

#### 命令工具

- rustc，rust编译器
  
  - rustc <fileName>，编译.rc文件，使得访问它可以直接执行

- rustup，rust管理器
  
  - rustup update，更新`rust`官方代码版本，达到最新

- cargo，rust项目管理器
  
  - cargo new <Name>，创建项目
  
  - cargo build，发布可调式版本代码
  
  - cargo build --release，发布正式版本代码
  
  - cargo clean，清理所有发布代码
  
  - cargo run，生成可调式版本代码，并运行
  
  - cargo run --quiet，生成可调式版本代码，并运行，不显示内部构建代码
  
  - cargo check，检查当前代码是否有编译上的问题，并生成可调式代码

- rustfmt，rust代码整理器
  
  - rustfmt <fileName>，格式化该文件的`rust`代码

#### 变量

- 变量声明，`let a` 
- 变量名，小写，用短横线隔开
- 变量可变性，默认声明的变量都不可变，想要改变就需要添加`mut`，例如`let mut data = 1;`
- 变量声明覆盖，重新声明名字相同的变量，将使最新声明的变量有效，之前的变量统统无效
- 变量作用域，内部作用域可以找外部作用域下的变量进行使用

#### 常量

- 变量声明，`const a:f64 = 7.25`，需要声明数据类型
