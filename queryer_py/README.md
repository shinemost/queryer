## windows 下测试

### 测试命令
```bash
    python -m venv .env # 创建虚拟环境
    .env\Scripts\active # 激活虚拟环境
    pip install maturin ipython # 安装依赖
    maturin develop # 使用maturin 开发模式编译源码与安装依赖
    ipython \ python # 进入交互命令界面
```

### Q&A

- project、package、filedir 名称里不能有-，否则会出编译错误
- pyo3 源码中的低版本编译会报错，只能使用最新版，某些API发生了变化，源码中做了修改，注意甄别。

### rust与python集成可参考pyo3库doc
[pyo3 doc](https://pyo3.rs/v0.23.3/)