# Tondi Opcode 工具库

一个为Tondi区块链opcode提供实用工具的Rust库。本模块扩展了`tondi-txscript` crate的功能，提供额外的辅助函数和宏，用于opcode操作和转换。

## 功能特性

- **Push Opcode检测**：识别push bytes和push data opcode的函数
- **字符串转换**：宏生成的函数，用于将字符串表示转换为opcode值
- **全面的Opcode支持**：涵盖所有标准Tondi opcode，包括数据操作、栈操作、算术运算和加密函数
- **别名支持**：常见opcode的多种字符串表示（例如："Op0"、"FALSE"、"TRUE"）

## 安装

在您的`Cargo.toml`中添加此依赖：

```toml
[dependencies]
tondi-opcode-utils = "0.17.0"
```

## 依赖关系

本库依赖于：
- `tondi-txscript = "0.17.0"` - Tondi交易脚本核心功能

## 使用方法

### 基本函数

```rust
use tondi_opcode_utils::{is_push_bytes, is_push_data};

// 检查opcode是否为push bytes opcode (0x01-0x4b)
let is_push = is_push_bytes(0x01); // true
let is_not_push = is_push_bytes(0x61); // false (OpNop)

// 检查opcode是否为push data opcode
let is_push_data = is_push_data(0x4c); // true (OP_PUSH_DATA1)
```

### 字符串到Opcode转换

```rust
use tondi_opcode_utils::from_str;

// 将字符串表示转换为opcode值
let opcode = from_str("Op0").unwrap(); // 返回 0x00
let opcode = from_str("TRUE").unwrap(); // 返回 0x51
let opcode = from_str("OpFalse").unwrap(); // 返回 0x00

// 支持带前缀和不带前缀两种格式
let opcode1 = from_str("OpAdd").unwrap(); // 返回 0x93
let opcode2 = from_str("Add").unwrap(); // 返回 0x93
```

### 可用的Opcode类别

#### 数据操作
- **Push操作**：`OpData1` 到 `OpData75`，`OP_PUSH_DATA1`，`OP_PUSH_DATA2`，`OP_PUSH_DATA4`
- **常量**：`OpFalse` (0x00)，`OpTrue` (0x51)，`Op1Negate` (0x4f)

#### 栈操作
- **复制**：`OpDup`，`Op2Dup`，`Op3Dup`，`OpIfDup`
- **移动**：`OpDrop`，`Op2Drop`，`OpNip`，`OpOver`，`OpPick`，`OpRoll`，`OpRot`，`OpSwap`，`OpTuck`
- **栈信息**：`OpDepth`，`OpSize`

#### 算术运算
- **基础数学**：`OpAdd`，`OpSub`，`OpMul`，`OpDiv`，`OpMod`
- **位运算**：`OpAnd`，`OpOr`，`OpXor`，`OpInvert`
- **比较**：`OpEqual`，`OpNumEqual`，`OpLessThan`，`OpGreaterThan`
- **递增/递减**：`Op1Add`，`Op1Sub`，`Op2Mul`，`Op2Div`

#### 控制流
- **条件语句**：`OpIf`，`OpNotIf`，`OpElse`，`OpEndIf`
- **验证**：`OpVerify`，`OpEqualVerify`，`OpNumEqualVerify`
- **时间锁**：`OpCheckLockTimeVerify`，`OpCheckSequenceVerify`

#### 加密函数
- **哈希**：`OpSHA256`，`OpBlake3`
- **签名**：`OpCheckSig`，`OpCheckSigVerify`，`OpCheckMultiSig`，`OpCheckMultiSigVerify`
- **ECDSA**：`OpCheckSigECDSA`，`OpCheckMultiSigECDSA`

## API参考

### 函数

#### `is_push_bytes(code: u8) -> bool`
判断给定的opcode是否为push bytes opcode (0x01-0x4b)。

#### `is_push_data(code: u8) -> bool`
判断给定的opcode是否为push data opcode之一 (0x4c, 0x4d, 0x4e)。

#### `from_str(s: &str) -> Result<u8, ()>`
将opcode的字符串表示转换为其字节值。如果字符串不被识别，返回`Err(())`。

### 常量

- `OP_PUSH_DATA1`：0x4c
- `OP_PUSH_DATA2`：0x4d  
- `OP_PUSH_DATA4`：0x4e

## 实现说明

- 由于对内部`Opcode`结构体值的访问受限，本库使用变通方法来确定opcode类别
- 字符串转换支持常见opcode的多种别名
- 所有opcode都定义为常量，包含其十六进制值
- 保留和未定义的opcode都进行了适当标记

## 许可证

本项目根据LICENSE文件中指定的条款进行许可。

## 贡献

欢迎贡献！请随时提交问题和拉取请求。

## 相关项目

- [Tondi](https://github.com/your-org/Tondi) - Tondi区块链核心实现
- [tondi-txscript](https://github.com/your-org/Tondi/tree/main/crypto/txscript) - Tondi交易脚本功能
