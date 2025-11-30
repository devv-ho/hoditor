# Command Pattern: Enum vs Trait Object

## Current Approach (Trait Objects)

```rust
trait Executable {
    fn execute(&self, context: &mut Context);
}

struct MoveCursorLeft;
impl Executable for MoveCursorLeft { ... }

struct MoveCursorRight;
impl Executable for MoveCursorRight { ... }

// Dispatcher returns: Box<dyn Executable>
// Problems:
// - Heap allocation for every command
// - Can't clone easily
// - Harder to serialize
// - Virtual dispatch overhead
```

## Enum Approach

```rust
enum Command {
    MoveCursor { dx: i32, dy: i32 },
    InsertChar(char),
    ChangeMode(Mode),
    Save,
}

impl Command {
    fn execute(&self, context: &mut Context) {
        match self {
            Command::MoveCursor { dx, dy } => { ... }
            Command::InsertChar(ch) => { ... }
            // ...
        }
    }
}

// Benefits:
// - Stack allocated
// - Clone is trivial
// - Exhaustive pattern matching
// - Can carry data in variants
// - No vtable overhead
```

## For hoditor

Instead of 20+ separate structs implementing Executable, you'd have:

```rust
enum Command {
    // Cursor movement
    MoveCursor { dx: i32, dy: i32 },

    // Editing
    InsertChar(char),
    InsertNewLine,
    RemoveChar,

    // Mode changes
    ChangeMode(Mode),

    // File operations
    Save,
    SaveAndRestart,

    // Special
    DoNothing,
    Terminate,
}
```

Much simpler! Would you like me to refactor hoditor to use this approach?
