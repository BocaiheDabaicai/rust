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

- rustup，rust管理器

- cargo，rust项目管理器
