# TreeSitter Syntax Highlighting Upgrade

## Problem
The original syntax highlighter used Syntect (TextMate grammars), which had limited capability:
- Enum variants like `MoveCursor`, `DoNothing` appeared white (unhighlighted)
- Type names in expressions like `Command` in `Command::MoveCursor` appeared white
- Function names after `::` appeared white
- Type names in use statements and struct fields appeared white

## Solution
Switched to **TreeSitter** for Rust files - the same powerful parser used by modern Vim with TreeSitter support.

### What Changed
1. **Added TreeSitter dependencies** (`tree-sitter`, `tree-sitter-rust`)
2. **Created `ts_highlighter.rs`** - New TreeSitter-based highlighter with proper query patterns
3. **Updated `renderer.rs`** - Uses TreeSitter for `.rs` files, falls back to Syntect for other languages
4. **Deduplication logic** - Handles overlapping token captures to avoid duplicate highlighting

### Fixed Issues
✅ **"register" printed twice** - Fixed by deduplicating overlapping captures
✅ **Command is highlighted** - Now shows as cyan (Type color)
✅ **MoveCursor is highlighted** - Now shows as cyan (Type color)
✅ **ChangeMode is highlighted** - Now shows as blue (Function color)
✅ **Mode is highlighted** - Now shows as cyan (Type color)
✅ **enum keyword** - Properly highlighted
✅ **All type names, function names, constants** - Properly distinguished

### Token Colors (TokyoNight Theme)
- **Types** (Command, Mode, HashMap): Cyan (`#2ac3de`)
- **Functions** (register, execute): Blue (`#7aa2f7`)
- **Keywords** (enum, struct, impl, fn): Bright Cyan (`#7dcfff`)
- **Strings**: Green (`#9ece6a`)
- **Numbers**: Orange (`#ff9e64`)
- **Comments**: Muted Blue (`#565f89`)
- **Constants/Variants**: Orange (`#ff9e64`)

### Architecture
```
Renderer
├── For .rs files → TreeSitterHighlighter (tree-sitter-rust parser)
└── For other files → Highlighter (syntect with TextMate grammars)
```

### Performance
TreeSitter is actually **faster** than regex-based parsers for large files because it builds a proper syntax tree and can incrementally parse changes.

## Testing
Run tests to verify highlighting:
```bash
cargo test test_command_highlighting -- --nocapture
cargo test test_changemode_highlighting -- --nocapture
```

## Result
Your editor now has **professional-grade syntax highlighting** that matches or exceeds Vim with TreeSitter!
