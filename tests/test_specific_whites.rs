use syntect::parsing::{ParseState, ScopeStack, SyntaxSet};

#[test]
fn analyze_specific_white_cases() {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();

    let test_cases = vec![
        (
            "Enum variant in definition",
            "    MoveCursor { dx: i32, dy: i32 },",
        ),
        ("Type in use statement", "use std::collections::HashMap;"),
        ("Type in struct field", "    dispatcher: CmdDispatcher,"),
        ("Function call", "CmdDispatcher::new()"),
        ("Enum variant in match", "Command::DoNothing => {}"),
        ("Variable assignment", "let mut dispatcher = value;"),
    ];

    println!("\n{:=<100}", "");
    println!("DETAILED ANALYSIS OF SPECIFIC WHITE TOKEN CASES");
    println!("{:=<100}", "");

    for (desc, line) in test_cases {
        println!("\n[{}]", desc);
        println!("Line: {}", line);
        println!("{:-<100}", "");

        let mut state = ParseState::new(syntax);
        let ops = state.parse_line(line, &syntax_set).unwrap();

        let mut scope_stack = ScopeStack::new();
        let mut col = 0;

        for (pos, op) in ops {
            if pos > col {
                let text = &line[col..pos];
                if !text.trim().is_empty() {
                    let scope_str = scope_stack.to_string();
                    let is_white = scope_str == "source.rust" || scope_str == "source.rust ";
                    println!(
                        "  {:30} | {} | {}",
                        format!("'{}'", text),
                        if is_white { "WHITE" } else { "colored" },
                        scope_str
                    );
                }
                col = pos;
            }
            scope_stack.apply(&op).unwrap();
        }

        if col < line.len() {
            let text = &line[col..];
            if !text.trim().is_empty() {
                let scope_str = scope_stack.to_string();
                let is_white = scope_str == "source.rust" || scope_str == "source.rust ";
                println!(
                    "  {:30} | {} | {}",
                    format!("'{}'", text),
                    if is_white { "WHITE" } else { "colored" },
                    scope_str
                );
            }
        }
    }

    println!("\n{:=<100}", "");
}
