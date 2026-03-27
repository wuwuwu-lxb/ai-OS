# AI-OS 🚀

<div align="center">

**An AI-Native Operating System built with Rust**

*为 AI Agent 设计的原生操作系统 - 不是为人类用户，而是为 AI 自主运行而构建*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Status-Active-green.svg)]()

</div>

---

## 🎯 项目概述

**AI-OS** 是一个专为 AI Agent 设计的原生操作系统。与传统操作系统不同，它不是为人类用户设计的，而是为 AI Agent 提供**自我扩展、系统集成和自主决策能力**的平台。

### 核心创新

- 🧠 **AI 原生设计** - 从头为 AI Agent 优化
- 🔄 **动态能力系统** - AI 可注册新的系统能力
- 📊 **向量数据库** - 原生支持语义搜索
- 🤖 **多 Agent 协作** - 支持并发 AI Agent
- ⚡ **低延迟接口** - 为 AI 提供高效系统调用

### 目标用户

- AI 研究者和开发者
- AI Agent 开发者
- 自主 AI 系统研究者
- Rust 嵌入式爱好者

---

## 📦 项目结构

```
AI-OS/
├── crates/                    # AI 用户空间应用 (运行在 Linux 上)
│   ├── aios-core            # 核心协调器
│   ├── aios-ai             # AI 中间件 (Claude/OpenAI)
│   ├── aios-cli            # 终端 REPL
│   ├── aios-memory         # 记忆管理
│   ├── aios-multimodal     # 多模态处理
│   ├── aios-fs             # 文件系统抽象
│   ├── aios-process        # 进程管理
│   ├── aios-network        # 网络通信
│   ├── aios-voice          # 语音合成
│   ├── aios-policy         # 权限策略
│   └── aios-kernel         # Linux 系统调用桥接
│
└── os/                      # 裸机内核 (x86_64) ⭐ 当前开发重点
    ├── src/
    │   ├── main.rs          # 内核入口
    │   ├── gdt.rs          # 全局描述符表
    │   ├── idt.rs          # 中断描述符表
    │   ├── pic.rs          # 可编程中断控制器
    │   ├── pit.rs          # 定时器
    │   ├── keyboard.rs     # 键盘驱动
    │   ├── serial.rs       # 串口驱动
    │   ├── vga.rs         # VGA 显示驱动
    │   ├── memory.rs       # 物理内存管理
    │   ├── paging.rs       # 虚拟内存/分页
    │   ├── heap.rs         # 堆内存分配
    │   ├── process.rs      # 进程管理
    │   ├── scheduler.rs     # 进程调度
    │   ├── ai_service.rs   # AI 服务框架
    │   ├── vector_db.rs    # 向量数据库
    │   ├── capability.rs    # 能力注册系统
    │   └── ai_core.rs      # AI 核心模块
    └── arch/
        └── x86_64/
            ├── boot.S     # 引导代码
            └── linker.ld   # 链接脚本
```

---

## ✨ 当前版本功能 (v0.5.0)

### Phase 1: 基础内核 ✅

#### 硬件抽象层
- ✅ **GDT 管理器** - 内存保护、特权级别
- ✅ **中断系统** - PIC、IDT、硬件/软中断
- ✅ **定时器驱动** - 100Hz PIT 定时器
- ✅ **键盘驱动** - PS/2 键盘支持
- ✅ **I/O 系统** - 串口、VGA

#### 内存管理
- ✅ **物理内存** - 内存检测和统计
- ✅ **虚拟内存** - 分页机制
- ✅ **堆分配器** - 16MB bump allocator

#### 进程管理
- ✅ **PCB 结构** - 进程控制块
- ✅ **调度器** - 轮转调度
- ✅ **上下文切换** - 寄存器保存/恢复

### Phase 2: AI 服务框架 ✅

#### AI 服务层
- ✅ **服务注册表** - 32 个服务槽位
- ✅ **生命周期管理** - 完整的启动/停止流程
- ✅ **状态跟踪** - 5 种服务状态

#### 向量数据库
- ✅ **存储引擎** - 1024 向量 × 128 维
- ✅ **相似度计算** - 余弦相似度 (libm)
- ✅ **CRUD 操作** - 增删改查完整支持

#### 能力系统
- ✅ **注册表** - 64 个能力槽位
- ✅ **能力类型** - Tool/Skill/Plugin/API
- ✅ **版本管理** - 启用/禁用能力
- ✅ **7 个默认能力**:
  - file_read, file_write
  - vector_search
  - process_create, process_list
  - memory_store, memory_retrieve

---

## 🏗️ 系统架构

```
┌─────────────────────────────────────────────────────┐
│          AI Agent Layer (用户空间)                  │
│   • AI进程管理  • 能力注册  • Agent间通信          │
├─────────────────────────────────────────────────────┤
│          AI Service Layer (系统服务) ⭐             │
│   • 向量数据库  • 知识图谱  • 意图理解  • 规划器   │
├─────────────────────────────────────────────────────┤
│          Kernel Abstraction Layer                   │
│   • 统一系统调用  • 虚拟文件系统  • 进程调度        │
├─────────────────────────────────────────────────────┤
│          Hardware Abstraction Layer                  │
│   • 内存管理  • 中断处理  • I/O  • 网络           │
├─────────────────────────────────────────────────────┤
│          Hardware (x86_64)                          │
└─────────────────────────────────────────────────────┘
```

---

## 🚀 快速开始

### 环境要求

- Rust nightly
- NASM 汇编器
- GRUB 工具链
- QEMU 模拟器
- 8GB+ RAM

### 构建步骤

```bash
# 1. 克隆仓库
git clone https://github.com/wuwuwu-lxb/ai-OS.git
cd AI-OS

# 2. 安装 Rust nightly
rustup nightly install
rustup target add x86_64-unknown-none --toolchain nightly

# 3. 编译内核
cd os
cargo +nightly build --release --target x86_64-unknown-none

# 4. 创建 ISO 镜像
cp target/x86_64-unknown-none/release/ai-os-kernel iso/boot/
grub-mkrescue -o ai-os.iso iso/

# 5. 运行测试
qemu-system-x86_64 -cdrom ai-os.iso -m 256
```

### 预期输出

```
==========================================
  AI-OS Kernel v0.5.0 - AI Services
==========================================

[OK] Serial port initialized
[OK] GDT initialized
[OK] Initializing interrupt system...
[OK] Initializing memory system...
[OK] Initializing process system...
[OK] Initializing AI Service Framework...

[INFO] AI Service Summary:
  Capabilities registered: 7
  Services registered: 0
  Vector storage: 0 vectors

[OK] AI-OS Kernel v0.5.0 is running!
[OK] AI Services: Active
```

---

## 📊 技术亮点

### 1. Rust no_std 环境
```rust
// 完全脱离标准库，裸机运行
#![no_std]
#![no_main]

// 静态分配，避免动态内存
static mut BUFFER: [u8; 1024] = [0; 1024];

// 核心库功能
use core::mem::size_of;
```

### 2. x86_64 架构
- ✅ 保护模式 (Ring 0-3)
- ✅ 分页内存管理
- ✅ 中断向量处理
- ✅ 特权级别分离

### 3. AI 原生设计
```rust
// 原生向量存储
let vector = [0.1f32; 128];
vector_db::insert(&vector, &metadata);

// 能力注册
capability::register(
    "vector_search",
    "Semantic search capability",
    CapabilityType::Tool
);
```

---

## 🎯 开发路线图

### 已完成 ✅
- ✅ **v0.1.0** - 初始版本
- ✅ **v0.2.0** - 中断系统
- ✅ **v0.3.0** - 内存管理
- ✅ **v0.4.0** - 进程调度
- ✅ **v0.5.0** - AI 服务框架 ⭐ 当前版本

### Phase 3: AI 原生特性 🚧
- [ ] 语义文件系统
- [ ] 知识图谱服务
- [ ] 意图理解引擎
- [ ] AI Agent 框架

### Phase 4: 高级特性 📋
- [ ] 多 Agent 协作
- [ ] 自我进化系统
- [ ] 神经接口优化

---

## 📚 文档

- 📖 **[项目总结](PROJECT_SUMMARY.md)** - 完整项目概述
- 📖 **[功能详解](FEATURES.md)** - 详细功能阐述
- 📖 **[开发日志](os/tasks.md)** - 任务分解和时间表
- 📖 **[检查清单](os/checklist.md)** - 开发检查清单
- 📖 **[测试指南](os/README.md)** - 内核测试方法

---

## 🧪 测试与调试

### QEMU 测试
```bash
# 图形界面
qemu-system-x86_64 -cdrom ai-os.iso -m 256

# 串口输出
qemu-system-x86_64 -cdrom ai-os.iso -nographic

# GDB 调试
qemu-system-x86_64 -cdrom ai-os.iso -s -S &
gdb target/x86_64-unknown-none/release/ai-os-kernel
(gdb) target remote localhost:1234
```

### 性能基准
```
内核大小: ~22KB
启动时间: < 1s
中断响应: ~1ms
向量搜索: O(n) 遍历
```

---

## 👥 贡献指南

欢迎贡献！请查看我们的路线图和待办事项。

### 贡献方式
- 🐛 Bug 报告
- 💡 功能建议
- 📖 文档改进
- 💻 代码贡献

### 开发规范
- 遵循 Rust 编码规范
- 使用 `cargo fmt`
- 添加必要的注释
- 编写测试用例

---

## 📝 许可证

本项目采用 **MIT** 许可证开源。

---

## 🔗 相关资源

- 📚 [Rust 嵌入式开发](https://rust-embedded.org/)
- 📚 [OSDev Wiki](https://wiki.osdev.org/)
- 📚 [Rustonomicon](https://doc.rust-lang.org/nomicon/)
- 📚 [Writing an OS in Rust](https://os.phil-opp.com/)

---

## 📞 联系方式

- 🐛 **Bug 报告**: [GitHub Issues](https://github.com/wuwuwu-lxb/ai-OS/issues)
- 💬 **讨论**: [GitHub Discussions](https://github.com/wuwuwu-lxb/ai-OS/discussions)
- 📧 **邮箱**: (待添加)

---

## 🎉 致谢

感谢所有为 AI-OS 做出贡献的开发者！

特别感谢：
- Rust 社区
- OSDev 社区
- 所有开源项目贡献者

---

<div align="center">

**加入我们的旅程，一起构建 AI 原生的未来！** 🚀

*AI-OS v0.5.0 - 让 AI Agent 拥有自己的操作系统*

</div>
