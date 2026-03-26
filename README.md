# AI-OS

An AI-Native Operating System built with Rust, featuring a bare metal kernel and AI-powered user experience.

## Project Structure

```
OS-ai/
├── crates/           # AI User Space Application (runs on Linux)
└── os/              # Bare Metal OS Kernel (x86_64)
```

## Two Components

### 1. AI User Space (`crates/`)

A Rust application that runs on Linux, providing Claude/OpenAI powered AI interaction.

**Features:**
- Terminal REPL interface
- Claude/OpenAI API integration
- Conversation memory and context
- Multi-modal input processing (text, voice, image, code)

**Build & Run:**
```bash
cd OS-ai
export ANTHROPIC_API_KEY="your-key-here"
cargo run --release
```

### 2. Bare Metal Kernel (`os/`)

A minimal x86_64 OS kernel written in Rust, designed to run directly on hardware.

**Current Features:**
- Multiboot2 compliant bootloader
- VGA text mode driver
- Basic kernel entry and halt

**Build (requires nightly Rust):**
```bash
cd os
rustup nightly install
rustup target add x86_64-unknown-none --toolchain nightly
make
```

**Test in QEMU (requires additional tools):**
```bash
# Install boot tools
sudo apt install mtools grub-pc

# Create bootable ISO
grub-mkrescue -o target/ai-os.iso target/iso

# Run in QEMU
qemu-system-x86_64 -cdrom target/ai-os.iso -nographic
```

## Architecture

### User Space Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                    User Interaction Layer                    │
│                    CLI (REPL)                               │
├─────────────────────────────────────────────────────────────┤
│                    AI Orchestration Layer                    │
│     Context Manager │ AI Middleware │ Cloud + Local         │
├─────────────────────────────────────────────────────────────┤
│                    System Services Layer                     │
│     Memory │ Filesystem │ Network │ Process                 │
├─────────────────────────────────────────────────────────────┤
│                      Linux Kernel                            │
└─────────────────────────────────────────────────────────────┘
```

### Kernel Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                    Bootloader (GRUB)                        │
├─────────────────────────────────────────────────────────────┤
│                    Kernel (Rust no_std)                    │
│     VGA Driver │ GDT │ Interrupts (future)                │
├─────────────────────────────────────────────────────────────┤
│                    User Space (future)                       │
│     AI Process │ Shell │ Applications                     │
└─────────────────────────────────────────────────────────────┘
```

## Crates (User Space)

| Crate | Description |
|-------|-------------|
| `aios-core` | Main orchestrator and entry point |
| `aios-ai` | AI middleware (Claude/OpenAI providers) |
| `aios-cli` | Terminal REPL interface |
| `aios-memory` | Conversation history and embeddings |
| `aios-multimodal` | Text, voice, image processing |
| `aios-fs` | Filesystem abstraction |
| `aios-process` | Process management |
| `aios-network` | HTTP client and API integrations |
| `aios-voice` | Text-to-speech output |
| `aios-policy` | Permission and security engine |
| `aios-kernel` | Linux syscall bridge |

## Roadmap

### User Space Application
- [x] Phase 1: Core foundation, CLI, AI integration
- [ ] Phase 2: Multi-modal input (voice, image)
- [ ] Phase 3: System services integration
- [ ] Phase 4: Memory and persistence
- [ ] Phase 5: Network and offline fallback
- [ ] Phase 6: Voice synthesis output
- [ ] Phase 7: Policy and security
- [ ] Phase 8: Self-iteration capabilities

### Bare Metal Kernel
- [x] Bootloader and kernel entry
- [ ] Interrupt handling
- [ ] Memory management (paging, heap)
- [ ] Keyboard driver
- [ ] Basic filesystem
- [ ] Process scheduler
- [ ] AI service as first user process
- [ ] AI-readable filesystem interface

## License

MIT OR Apache-2.0
