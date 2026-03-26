# AI-OS

An AI-Native Operating System built with Rust, where AI is not just an application but the core interface of the entire system.

## Overview

AI-OS reimagines the traditional operating system experience by placing a large language model as the central orchestrator. Instead of typing commands into a terminal or clicking on icons, users interact with their computer through natural conversation. The AI understands intent, executes tasks, manages files, and continuously learns from interactions.

## Key Features

- **AI-First Interface**: Natural language is the primary way to interact with the system
- **Multi-Modal Support**: Text, voice, images, and code inputs
- **Cloud-First AI**: Leverages Claude/OpenAI APIs with local fallback for resilience
- **Memory & Context**: Persistent conversation history and semantic memory
- **Self-Iterative**: The system can improve itself through AI-generated enhancements

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interaction Layer                    │
│        CLI (REPL)  │  Voice I/O  │  GUI (future)          │
├─────────────────────────────────────────────────────────────┤
│                  Multi-Modal Processing Layer                │
│           Text │ Voice (STT) │ Image │ Code                 │
├─────────────────────────────────────────────────────────────┤
│                    AI Orchestration Layer                    │
│  ┌─────────────────────────────────────────────────────┐    │
│  │   Context Manager │ AI Middleware │ Local Fallback   │    │
│  └─────────────────────────────────────────────────────┘    │
├─────────────────────────────────────────────────────────────┤
│                    System Services Layer                     │
│     Filesystem │ Process │ Memory │ Network │ Policy        │
├─────────────────────────────────────────────────────────────┤
│                      Linux Kernel                            │
└─────────────────────────────────────────────────────────────┘
```

## Crates

| Crate | Description |
|-------|-------------|
| `aios-core` | Main orchestrator and entry point |
| `aios-ai` | AI middleware (Claude/OpenAI/Local providers) |
| `aios-cli` | Terminal REPL interface |
| `aios-memory` | Conversation history and embeddings |
| `aios-multimodal` | Text, voice, image processing |
| `aios-fs` | Filesystem abstraction |
| `aios-process` | Process management |
| `aios-network` | HTTP client and API integrations |
| `aios-voice` | Text-to-speech output |
| `aios-policy` | Permission and security engine |
| `aios-kernel` | Linux syscall bridge |

## Getting Started

### Prerequisites

- Rust 1.70+
- Linux (for full functionality)
- API keys: `ANTHROPIC_API_KEY` or `OPENAI_API_KEY`

### Build

```bash
cargo build --release
```

### Run

```bash
# Set API key
export ANTHROPIC_API_KEY="your-key-here"

# Run
cargo run --release
```

### Usage

Once running, you'll see the AI-OS terminal:

```
╔═══════════════════════════════════════╗
║           AI-OS Terminal               ║
║   Your AI-Native Operating System     ║
╚═══════════════════════════════════════╝

Type your message and press Enter.
Press Ctrl+C or Ctrl+D to exit.

❯ Hello, what can you do?
```

## Configuration

Create `.aios/config.toml` to customize settings:

```toml
[ai]
primary_provider = "claude"
default_model = "claude-sonnet-4-20250514"
max_tokens = 4096
temperature = 0.7

[memory]
context_window_tokens = 128000
short_term_retention_days = 7

[cli]
prompt = "❯ "
show_timestamps = true
```

## Roadmap

- [x] Phase 1: Core foundation, CLI, AI integration
- [ ] Phase 2: Multi-modal input (voice, image)
- [ ] Phase 3: System services (filesystem, process management)
- [ ] Phase 4: Memory and persistence
- [ ] Phase 5: Network and offline fallback
- [ ] Phase 6: Voice synthesis output
- [ ] Phase 7: Policy and security
- [ ] Phase 8: Self-iteration capabilities

## License

MIT OR Apache-2.0
