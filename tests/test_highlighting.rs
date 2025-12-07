use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

#[test]
fn test_rust_scopes() {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();
    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes["base16-ocean.dark"];

    let mut h = HighlightLines::new(syntax, theme);

    // Test line from your code
    let line = r#"        dispatcher.register("h", Command::MoveCursor { dx: -1, dy: 0 });"#;

    let ranges = h.highlight_line(line, &syntax_set).unwrap();

    println!("\nScopes for line: {}", line);
    println!("{:-<80}", "");

    for (style, text) in ranges {
        if !text.trim().is_empty() {
            println!(
                "Token: {:20} | Scope: {:?}",
                format!("'{}'", text),
                style.foreground
            );
        }
    }
}

#[test]
fn test_rust_scopes_detailed() {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();

    // We need to track scopes manually
    use syntect::parsing::ScopeStack;
    use syntect::parsing::SyntaxSetBuilder;

    let ps = syntax_set.find_syntax_by_extension("rs").unwrap();

    let test_lines = vec![
        r#"dispatcher.register("h", Command::MoveCursor { dx: -1, dy: 0 });"#,
        r#"enum Command {"#,
        r#"    MoveCursor { dx: i32, dy: i32 },"#,
        r#"}"#,
        r#"Command::ChangeMode(Mode::Edit)"#,
    ];

    println!("\n{:=<80}", "");
    println!("DETAILED SCOPE ANALYSIS");
    println!("{:=<80}", "");

    for line in test_lines {
        println!("\nLine: {}", line);
        println!("{:-<80}", "");

        let mut state = syntect::parsing::ParseState::new(ps);
        let ops = state.parse_line(line, &syntax_set).unwrap();

        let mut scope_stack = ScopeStack::new();
        let mut col = 0;

        for (pos, op) in ops {
            if pos > col {
                let text = &line[col..pos];
                if !text.trim().is_empty() {
                    println!(
                        "  Token: {:25} | Scopes: {}",
                        format!("'{}'", text),
                        scope_stack
                    );
                }
                col = pos;
            }
            scope_stack.apply(&op).unwrap();
        }

        // Print remaining text
        if col < line.len() {
            let text = &line[col..];
            if !text.trim().is_empty() {
                println!(
                    "  Token: {:25} | Scopes: {}",
                    format!("'{}'", text),
                    scope_stack
                );
            }
        }
    }

    println!("\n{:=<80}", "");
}

#[test]
fn test_enum_keyword_scope() {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let syntax = syntax_set.find_syntax_by_extension("rs").unwrap();

    let test_lines = vec![
        "enum Command {",
        "struct Foo;",
        "impl Command {",
        "trait Executable {",
        "pub enum Mode {",
    ];

    println!("\n{:=<80}", "");
    println!("KEYWORD SCOPE ANALYSIS");
    println!("{:=<80}", "");

    for line in test_lines {
        println!("\nLine: {}", line);
        println!("{:-<80}", "");

        let mut state = syntect::parsing::ParseState::new(syntax);
        let ops = state.parse_line(line, &syntax_set).unwrap();

        let mut scope_stack = syntect::parsing::ScopeStack::new();
        let mut col = 0;

        for (pos, op) in ops {
            if pos > col {
                let text = &line[col..pos];
                if !text.trim().is_empty() {
                    println!(
                        "  Token: {:15} | Scopes: {}",
                        format!("'{}'", text),
                        scope_stack
                    );
                }
                col = pos;
            }
            scope_stack.apply(&op).unwrap();
        }

        if col < line.len() {
            let text = &line[col..];
            if !text.trim().is_empty() {
                println!(
                    "  Token: {:15} | Scopes: {}",
                    format!("'{}'", text),
                    scope_stack
                );
            }
        }
    }

    println!("\n{:=<80}", "");
}
