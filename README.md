# Bolt - 高性能负载测试工具

## 安装

```bash
cargo install bolt
```

## 使用方法

### API 调试

```bash
bolt debug --url "https://httpbin.org/get" --method GET
```

### 负载测试

```bash
# 基础测试
bolt load-test --url "https://httpbin.org/get" --concurrent 100 --duration 30

# 高并发测试
bolt load-test --url "https://api.example.com/users" --concurrent 10000 --duration 60
```

## 开发

```bash
# 开发模式运行
cargo run -- debug --url "https://httpbin.org/get"

# 发布版本
cargo build --release

# 运行测试
cargo test
```