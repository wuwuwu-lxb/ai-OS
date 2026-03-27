# AI-OS 测试指南

## 编译内核

```bash
cd os
cargo +nightly build --release --target x86_64-unknown-none
```

## 创建 Bootable ISO

```bash
# 复制内核到 boot 目录
cp target/x86_64-unknown-none/release/ai-os-kernel iso/boot/

# 创建 ISO
grub-mkrescue -o ai-os.iso iso/
```

## 在 QEMU 中测试

### 方法 1：使用图形界面（推荐）
```bash
qemu-system-x86_64 -cdrom ai-os.iso -m 256
```

### 方法 2：使用串口输出
```bash
# 创建命名管道
mkfifo /tmp/serial.out

# 在一个终端运行 QEMU
qemu-system-x86_64 -cdrom ai-os.iso -serial file:/tmp/serial.out -m 256 &

# 在另一个终端查看输出
cat /tmp/serial.out
```

### 方法 3：使用 nographic 模式
```bash
# 使用屏幕输出（VGA）
qemu-system-x86_64 -cdrom ai-os.iso -nographic -m 256
```

## 调试

### 启用 GDB 调试
```bash
# 终端 1：启动 QEMU 等待调试
qemu-system-x86_64 -cdrom ai-os.iso -s -S -nographic

# 终端 2：启动 GDB
gdb
(gdb) target remote localhost:1234
(gdb) symbol-file target/x86_64-unknown-none/release/ai-os-kernel
(gdb) break _start
(gdb) continue
```

## 常见问题

### Q: 内核编译失败？
A: 确保使用 nightly 工具链：`rustup default nightly`

### Q: QEMU 无法加载内核？
A: 检查是否正确创建了 multiboot2 头部。可以使用 `objdump -h ai-os-kernel` 查看。

### Q: 串口没有输出？
A: 确保串口驱动正确初始化，或使用 VGA 输出测试。
