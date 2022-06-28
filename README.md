# Rust HPC Exercise

## 开发指南

### 执行基准测试

```shell
cargo bench
```
会在 target/criterion/report 目录下生成一个整体的报告 index.html

### 生成:fire:图
``` shell
cargo install flamegraph
# 将生成一个 flamegraph.svg 图片
sudo cargo flamegraph --bench bench_main -- --bench
```

### 生成汇编代码

```shell
# 安装 cargo-asm
cargo install cargo-asm
# 查看 sum_array_with_bit_operator 方法的汇编代码
cargo asm rust_hpc_exercise::sum_array::sum_array_with_bit_operator
```

## 参考资料

1. [Rust性能评估与调优实践](https://zhuanlan.zhihu.com/p/451184900)
2. [查看 Golang、Lua、JS、Rust、Python等语言生成的汇编代码](https://zhuanlan.zhihu.com/p/77158150)
3. [Compiler intrinsics](https://doc.rust-lang.org/std/intrinsics/index.html)
4. [Rust 版本历史](https://rust-lang.github.io/rustup-components-history/)
5. [工欲性能调优，必先利其器（2）- 火焰图](https://pingcap.com/zh/blog/flame-graph)
