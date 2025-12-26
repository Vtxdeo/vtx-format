# vtx-format

[![Crates.io](https://img.shields.io/crates/v/vtx-format.svg)](https://crates.io/crates/vtx-format)
[![Documentation](https://docs.rs/vtx-format/badge.svg)](https://docs.rs/vtx-format)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**vtx-format** 定义了 VTX 插件系统使用的私有二进制容器格式 (`.vtx`)。

它负责将标准的 WebAssembly Component (`.wasm`) 封装为带有魔数（Magic Header）和版本控制的 VTX 插件包，确保运行时加载的是合法且版本兼容的插件。

## 格式规范

目前仅支持 **V1** 版本格式。所有多字节整数均采用 **大端序 (Big-Endian)**（注：当前版本仅涉及单字节版本号，无需字节序处理）。

### V1 结构布局

文件头固定为 4 字节，后跟原始的 Wasm Component 字节流。

| 偏移 (Offset) | 长度 (Bytes) | 字段 (Field) | 值 / 说明 (Value/Description) |
| :--- | :--- | :--- | :--- |
| **0x00** | 3 | **Magic Prefix** | `0x56 0x54 0x58` (ASCII: "VTX") |
| **0x03** | 1 | **Version** | `0x01` (当前版本) |
| **0x04** | N | **Payload** | 原始 WebAssembly Component 二进制数据 |

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
vtx-format = "0.1"

```

## 使用示例

### 编码

将原始 Wasm 字节封装为 `.vtx` 格式：

```rust
use vtx_format::encode_v1;

fn main() {
    let wasm_bytes = std::fs::read("plugin.wasm").unwrap();
    
    // 自动添加 "VTX\x01" 头
    let vtx_data = encode_v1(&wasm_bytes);
    
    std::fs::write("plugin.vtx", vtx_data).unwrap();
}

```

### 解码

解析 `.vtx` 文件并验证头部：

```rust
use vtx_format::{decode, VtxFormatError};

fn main() -> Result<(), VtxFormatError> {
    let data = std::fs::read("plugin.vtx").unwrap();

    // 返回 (版本号, 原始 Payload 切片)
    let (version, payload) = decode(&data)?;

    println!("Detected VTX version: {}", version);
    println!("Payload size: {} bytes", payload.len());
    
    Ok(())
}

```

## 错误处理

库提供了 `VtxFormatError` 枚举来处理解析错误：

* `TooShort`: 数据长度不足 4 字节。
* `InvalidPrefix`: 头部不是 "VTX"。
* `UnsupportedVersion(u8)`: 遇到了未知的版本号。

## 许可证 (License)

MIT License