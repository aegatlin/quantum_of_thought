# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Quantum of Thought is a note capture tool with real-time synchronization across multiple clients. The architecture uses CRDT (Conflict-free Replicated Data Types) via Automerge for distributed note editing without conflicts.

## Architecture

The system consists of four main components:

1. **crdt_note** (Rust/WASM): Core CRDT note implementation
   - Uses Automerge for conflict-free merging
   - Compiled to WASM for web client usage
   - Also used directly by CLI client
   - Each note has a UUIDv7 id and text content

2. **client_web** (React + TypeScript + Vite): Web interface
   - Uses the CRDT WASM module via `crdt_note` package
   - Three-layer data architecture:
     - **NoteService**: Synchronizes notes across storage and networks (HTTP + WebSocket)
     - **NoteStore**: React-aware external store using useSyncExternalStore API
     - **React Components**: UI layer consuming NoteStore
   - All public APIs are synchronous; async operations notify listeners when complete
   - Uses localForage for client-side persistence
   - Supports multiple network transports simultaneously

3. **client_cli** (Rust): Command-line interface
   - Binary name: `qot`
   - Uses the crdt_note library directly (not WASM)
   - Provides local note management

4. **server_web** (Elixir/Phoenix): Backend server
   - **Authentication**: Passwordless magic links with JWT access tokens (1hr) and refresh tokens (30 days)
   - **Main contexts**:
     - `Qot.Accounts` - user management, magic link auth, token operations
     - `Qot.Notes` - note operations with per-user isolation
   - **Per-user note isolation**: All notes scoped by user_id, WebSocket channels are `notes:user:#{user_id}`
   - **Storage adapter pattern**: `Qot.Storage.Adapter` behavior (currently ETS with composite keys `{user_id, note_id}`)
   - **PubSub architecture**: All note operations broadcast to per-user topics `notes:user:#{user_id}`
   - **WebSocket authentication**: JWT token required in socket connect params, verified before channel join

## Development Commands

### crdt_note (Rust WASM)

Build WASM package (required before web client can use it):
```sh
cd crdt_note
wasm-pack build .
```

This creates `crdt_note/pkg/` directory which is consumed by `client_web` as a file dependency.

Run Rust tests:
```sh
cd crdt_note
cargo test
```

### client_web (React)

Install dependencies (includes the WASM package from `../crdt_note/pkg`):
```sh
cd client_web
npm install
```

Development server:
```sh
cd client_web
npm run dev
```

Build:
```sh
cd client_web
npm run build
```

Lint:
```sh
cd client_web
npm run lint
```

Run unit tests (Vitest):
```sh
cd client_web
npm test
```

Run e2e tests (Playwright):
```sh
cd client_web
npm run test:e2e
```

Run e2e tests with UI:
```sh
cd client_web
npm run test:e2e:ui
```

Debug e2e tests:
```sh
cd client_web
npm run test:e2e:debug
```

### client_cli (Rust)

Install locally:
```sh
cd client_cli
cargo install --path .
```

This installs the `qot` binary.

Run tests:
```sh
cd client_cli
cargo test
```

### server_web (Elixir/Phoenix)

**Environment Setup:**
Uses `mise` for tool and environment management. Required environment variables in `mise.local.toml`:
- `DATABASE_URL` - PostgreSQL connection (e.g., postgresql://localhost/qot_dev)
- `FRONTEND_URL` - Frontend URL for magic links (e.g., http://localhost:5173)
- `JWT_SECRET_KEY` - Secret key for signing JWT tokens
- `RESEND_API_KEY` - API key from resend.com for sending emails

Get dependencies:
```sh
cd server_web
mix deps.get
```

Run database migrations:
```sh
cd server_web
mix ecto.migrate
```

Start server in interactive mode:
```sh
cd server_web
iex -S mix phx.server
```

Run all tests:
```sh
cd server_web
mix test
```

Run specific test file:
```sh
cd server_web
mix test test/path/to/test_file.exs
```

Run specific test at line:
```sh
cd server_web
mix test test/path/to/test_file.exs:42
```

## Key Implementation Details

### WASM Integration
- The web client uses `vite-plugin-wasm` and `vite-plugin-top-level-await` for WASM support
- WASM boundary: Pass primitives and use non-mutable borrows to avoid gotchas
- The WASM package is linked as a file dependency in `client_web/package.json`: `"crdt_note": "file:../crdt_note/pkg"`

### Network Synchronization
- Web client supports multiple simultaneous network transports (HTTP + WebSocket)
- NoteService handles incoming messages from all networks and deduplicates
- All CRUD operations broadcast to all configured networks
- Server uses Phoenix PubSub to broadcast changes to all connected WebSocket clients
- Note data is serialized as Automerge binary format and transmitted as base64 over WebSocket

### Data Flow (Web Client)
1. User action → Component
2. Component calls NoteStore method
3. NoteStore calls NoteService method
4. NoteService:
   - Updates in-memory WASM note
   - Persists to localForage
   - Broadcasts to all networks (HTTP + WebSocket)
   - Notifies listeners
5. NoteStore updates snapshot and notifies React
6. React re-renders components

### Authentication System (Server)
- **Passwordless authentication**: Magic links sent via email (15 min expiry)
- **Token architecture**:
  - Access tokens: JWT, 1 hour expiry, contain user_id claim
  - Refresh tokens: Random, 30 days expiry, hashed in database
  - Magic link tokens: Random, 15 min expiry, hashed in database
- **Auth flow**:
  1. User requests magic link → stored in `magic_link_tokens` table
  2. User clicks link → creates/finds user, issues JWT + refresh token
  3. JWT used for API/WebSocket auth, refresh token for renewal
- **WebSocket auth**: JWT passed in socket connect params, user_id extracted and assigned to socket
- **HTTP auth**: `RequireAuth` plug checks Bearer token, assigns user_id to conn

### Per-User Note Isolation (Server)
- All storage operations require `user_id` parameter
- ETS keys are composite: `{user_id, note_id}`
- PubSub topics are per-user: `"notes:user:#{user_id}"`
- WebSocket channels enforce user matching: `"notes:user:#{user_id}"` only joinable by that user
- Notes context filters all queries by user_id

### Storage Adapter Pattern (Server)
- `Qot.Storage.Adapter` defines the behavior with user_id in all callbacks
- Current implementation: `Qot.Storage.EtsAdapter` (in-memory with composite keys)
- Configured via `config/config.exs`: `config :qot, :storage_adapter, Qot.Storage.EtsAdapter`
- All storage operations return `{:ok, result}` or `{:error, reason}` tuples

### Testing (Server)
- Uses Ecto.Adapters.SQL.Sandbox for test database isolation
- Test email adapter: `Swoosh.Adapters.Test` for magic link emails
- Test helpers in `test/support/`:
  - `DataCase` - sets up SQL Sandbox for all database tests
  - `ConnCase` - HTTP request testing with SQL Sandbox
  - `ChannelCase` - WebSocket channel testing with SQL Sandbox
  - `AuthHelpers` - creates authenticated users with JWT tokens
- ETS table requires manual cleanup in tests: `:ets.delete_all_objects(:qot_notes)`
- Mix automatically sets `MIX_ENV=test` when running `mix test`
