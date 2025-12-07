use syntect::parsing::{ParseState, ScopeStack, SyntaxSet};

#[test]
fn find_all_white_tokens() {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();

    let test_code = vec![
        // Basic enum usage
        r#"enum Command {"#,
        r#"    MoveCursor { dx: i32, dy: i32 },"#,
        r#"    InsertChar(char),"#,
        r#"    DoNothing,"#,
        r#"}"#,
        r#""#,
        // Struct
        r#"struct EventHandler {"#,
        r#"    dispatcher: CmdDispatcher,"#,
        r#"}"#,
        r#""#,
        // Impl block
        r#"impl EventHandler {"#,
        r#"    pub fn new() -> Self {"#,
        r#"        let mut dispatcher = CmdDispatcher::new();"#,
        r#"        dispatcher.register("h", Command::MoveCursor { dx: -1, dy: 0 });"#,
        r#"        dispatcher.register("i", Command::ChangeMode(Mode::Edit));"#,
        r#"        Self { dispatcher }"#,
        r#"    }"#,
        r#"}"#,
        r#""#,
        // Match expression
        r#"match self {"#,
        r#"    Command::DoNothing => {},"#,
        r#"    Command::MoveCursor { dx, dy } => {"#,
        r#"        cursor.move_left(1);"#,
        r#"    }"#,
        r#"    Command::ChangeMode(mode) => {"#,
        r#"        context.app_state.set_mode(*mode);"#,
        r#"    }"#,
        r#"}"#,
        r#""#,
        // Type annotations
        r#"fn handle(&mut self, event: Event, mode: Mode) -> Command {"#,
        r#"    Command::DoNothing"#,
        r#"}"#,
        r#""#,
        // Trait
        r#"trait Executable {"#,
        r#"    fn execute(&self, context: &mut Option<Context>);"#,
        r#"}"#,
        r#""#,
        // Use statements
        r#"use std::collections::HashMap;"#,
        r#"use crate::app::Context;"#,
        r#""#,
        // Derive macros
        r#"#[derive(Debug, Clone)]"#,
        r#"pub enum Mode {"#,
        r#"    Normal,"#,
        r#"    Edit,"#,
        r#"}"#,
        r#""#,
        // More complex
        r#"let result = self.root.find(&self.query);"#,
        r#"self.dispatcher.get().unwrap_or(Command::DoNothing)"#,
        r#"context.cursor.set_style(CursorStyle::Block);"#,
    ];

    println!("\n{:=<100}", "");
    println!("FINDING ALL TOKENS WITH ONLY 'source.rust' SCOPE (WHITE/UNHIGHLIGHTED)");
    println!("{:=<100}", "");

    for line in test_code {
        if line.is_empty() {
            continue;
        }

        let mut state = ParseState::new(syntax);
        let ops = state.parse_line(line, &syntax_set).unwrap();

        let mut scope_stack = ScopeStack::new();
        let mut col = 0;
        let mut white_tokens = Vec::new();

        for (pos, op) in ops {
            if pos > col {
                let text = &line[col..pos];
                let scope_str = scope_stack.to_string();

                // Check if it only has source.rust (no other scopes)
                if !text.trim().is_empty()
                    && (scope_str == "source.rust"
                        || scope_str == "source.rust "
                        || !scope_str.contains(" ") && scope_str == "source.rust")
                {
                    white_tokens.push(text.to_string());
                }
                col = pos;
            }
            scope_stack.apply(&op).unwrap();
        }

        if col < line.len() {
            let text = &line[col..];
            let scope_str = scope_stack.to_string();
            if !text.trim().is_empty()
                && (scope_str == "source.rust"
                    || scope_str == "source.rust "
                    || !scope_str.contains(" ") && scope_str == "source.rust")
            {
                white_tokens.push(text.to_string());
            }
        }

        if !white_tokens.is_empty() {
            println!("\nLine: {}", line);
            println!("  White tokens: {:?}", white_tokens);
        }
    }

    println!("\n{:=<100}", "");
}

#[test]
fn categorize_all_scopes() {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();

    let test_code = vec![
        r#"dispatcher.register("h", Command::MoveCursor { dx: -1, dy: 0 });"#,
        r#"enum Command { DoNothing, MoveCursor { dx: i32, dy: i32 } }"#,
        r#"impl Command { fn execute(&self) {} }"#,
        r#"match self { Command::DoNothing => {}, _ => {} }"#,
        r#"let x: Option<String> = Some(String::from("test"));"#,
        r#"pub struct EventHandler { dispatcher: CmdDispatcher }"#,
        r#"#[derive(Debug, Clone)]"#,
        r#"use std::collections::HashMap;"#,
        r#"fn handle(&mut self, event: Event) -> Command {"#,
    ];

    use std::collections::HashMap;
    let mut scope_map: HashMap<String, Vec<String>> = HashMap::new();

    println!("\n{:=<100}", "");
    println!("ALL UNIQUE SCOPES FOUND");
    println!("{:=<100}", "");

    for line in test_code {
        let mut state = ParseState::new(syntax);
        let ops = state.parse_line(line, &syntax_set).unwrap();

        let mut scope_stack = ScopeStack::new();
        let mut col = 0;

        for (pos, op) in ops {
            if pos > col {
                let text = &line[col..pos];
                if !text.trim().is_empty() {
                    let scope_str = scope_stack.to_string();
                    scope_map
                        .entry(scope_str.clone())
                        .or_insert_with(Vec::new)
                        .push(text.to_string());
                }
                col = pos;
            }
            scope_stack.apply(&op).unwrap();
        }

        if col < line.len() {
            let text = &line[col..];
            if !text.trim().is_empty() {
                let scope_str = scope_stack.to_string();
                scope_map
                    .entry(scope_str.clone())
                    .or_insert_with(Vec::new)
                    .push(text.to_string());
            }
        }
    }

    let mut scopes: Vec<_> = scope_map.iter().collect();
    scopes.sort_by_key(|(k, _)| k.as_str());

    for (scope, examples) in scopes {
        println!("\nScope: {}", scope);
        let sample: Vec<_> = examples.iter().take(5).map(|s| s.as_str()).collect();
        println!("  Examples: {:?}", sample);
    }

    println!("\n{:=<100}", "");
}
