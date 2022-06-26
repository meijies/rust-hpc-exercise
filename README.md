# Rust HPC Exercise

## 开发指南

### 生成汇编代码

```shell
cargo rustc -- --emit asm
```

### 生成优化的汇编代码

```shell
cargo rustc --release --emit asm
```

## 参考资料

1. [Rust性能评估与调优实践](https://zhuanlan.zhihu.com/p/451184900)
2. [查看 Golang、Lua、JS、Rust、Python等语言生成的汇编代码](https://zhuanlan.zhihu.com/p/77158150)
3. [Compiler intrinsics](https://doc.rust-lang.org/std/intrinsics/index.html)
4. [Rust 版本历史](https://rust-lang.github.io/rustup-components-history/)
