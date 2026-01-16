# MALAX

Lightweight MLX-LM alternative exposing **Ollama-like** cli tool and endpoints and **OpenAI-compatible** `/v1` APIs so existing tooling and OpenAI client libraries can work with minimal changes.

## Features

* OpenAI-compatible REST endpoints (`/v1/chat/completions`, `/v1/completions`, `/v1/embeddings`, `/v1/models`)
* Ollama-style endpoints (`/api/generate`, `/api/models`) for parity with Ollama clients
* Pluggable model backend (local models, container runtime, or remote LLM host)
* Simple API key auth and configurable rate limits
* JSON responses and streaming support for chat/completions
* Minimal dependencies; easy to run in Docker or as an ASGI app

## Requirements

* macOS 12.0+ (Apple Silicon recommended for performance)
* Memory: 8GB+ RAM (16GB+ recommended for larger models)

## Installation

### Download App

1. Go to the [Releases](https://github.com/Maanas-Verma/malax/releases) page.
2. Download `Malax.dmg`.
3. Open the disk image and drag **Malax** to your **Applications** folder.
4. Open **Malax** from your Applications or Spotlight.

*Note: You might need to allow the application in "Privacy & Security" settings if not signed by an identified developer yet.*

## Usage

Malax runs as a **Background Application** with a System Tray (Menu Bar) icon.

1. **Start Malax**: The icon will appear in your top menu bar.
2. **Open Dashboard**: Click the Malax icon -> **Open Malax**. This launches the chat interface and model manager.
3. **Configuration**: Click the Malax icon -> **Settings** to configure:
   - Model Paths
   - Default Port (default: 8080)
   - API Keys
4. **API Access**: The server starts automatically. You can access it at `http://localhost:8080`.

### Menu Bar Options
- **Open Malax**: Opens the main user interface.
- **Settings**: Opens preferences.
- **Quit Malax**: Stops the server and exits the application.

## Development

Prerequisites:
- Rust (latest stable)
- Node.js (for frontend building)

### Setup

```bash
git clone https://github.com/your-org/malax.git
cd malax
npm install
```

### Run Locally

```bash
# Starts the Tauri application in development mode
npm run tauri dev
```

### Build for Production

```bash
# Builds the .app and .dmg bundles
npm run tauri build
```

## Contributing

* Fork, create feature branch, open PR
* Follow existing API contracts; add tests for new endpoints
* Include documentation for any breaking changes

## Troubleshooting

* If model fails to load, check `MODEL_PATH` and backend logs
* For streaming failures, ensure client supports chunked transfer
* Rate-limit rejects return `429` with `Retry-After` header

## License

MIT License — see `LICENSE` file.
