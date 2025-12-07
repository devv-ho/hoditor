use hoditor::ts_highlighter::{TokenType, TreeSitterHighlighter};

#[test]
fn test_command_highlighting() {
    let mut highlighter = TreeSitterHighlighter::new();

    // Test the exact line from EventHandler::new()
    let line = r#"        dispatcher.register("h", Command::MoveCursor { dx: -1, dy: 0 });"#;

    let tokens = highlighter.highlight_line(line);

    println!("\n{:=<100}", "");
    println!("Testing: {}", line);
    println!("{:=<100}", "");

    // Print all tokens
    for (start, end, token_type) in &tokens {
        let text = &line[*start..*end];
        println!("  '{}'  =>  {:?}", text, token_type);
    }

    // Check that "register" appears only once
    let register_tokens: Vec<_> = tokens
        .iter()
        .filter(|(start, end, _)| &line[*start..*end] == "register")
        .collect();

    println!("\n'register' appears {} time(s)", register_tokens.len());
    assert_eq!(
        register_tokens.len(),
        1,
        "register should appear exactly once"
    );

    // Check that "Command" is highlighted as Type
    let command_token = tokens
        .iter()
        .find(|(start, end, _)| &line[*start..*end] == "Command");

    assert!(command_token.is_some(), "Command should be highlighted");
    let (_, _, token_type) = command_token.unwrap();
    println!("'Command' is highlighted as: {:?}", token_type);
    assert_eq!(*token_type, TokenType::Type, "Command should be Type");

    // Check that "MoveCursor" is present
    let move_cursor_token = tokens
        .iter()
        .find(|(start, end, _)| &line[*start..*end] == "MoveCursor");

    assert!(
        move_cursor_token.is_some(),
        "MoveCursor should be highlighted"
    );
    let (_, _, token_type) = move_cursor_token.unwrap();
    println!("'MoveCursor' is highlighted as: {:?}", token_type);

    println!("\n{:=<100}", "");
}

#[test]
fn test_changemode_highlighting() {
    let mut highlighter = TreeSitterHighlighter::new();

    let line = r#"        dispatcher.register("i", Command::ChangeMode(Mode::Edit));"#;

    let tokens = highlighter.highlight_line(line);

    println!("\n{:=<100}", "");
    println!("Testing: {}", line);
    println!("{:=<100}", "");

    for (start, end, token_type) in &tokens {
        let text = &line[*start..*end];
        println!("  '{}'  =>  {:?}", text, token_type);
    }

    // Check Command
    let command_token = tokens
        .iter()
        .find(|(start, end, _)| &line[*start..*end] == "Command");
    assert!(command_token.is_some(), "Command should be highlighted");
    assert_eq!(command_token.unwrap().2, TokenType::Type);

    // Check ChangeMode
    let changemode_token = tokens
        .iter()
        .find(|(start, end, _)| &line[*start..*end] == "ChangeMode");
    assert!(
        changemode_token.is_some(),
        "ChangeMode should be highlighted"
    );

    // Check Mode
    let mode_token = tokens
        .iter()
        .find(|(start, end, _)| &line[*start..*end] == "Mode");
    assert!(mode_token.is_some(), "Mode should be highlighted");
    assert_eq!(mode_token.unwrap().2, TokenType::Type);

    println!("\n{:=<100}", "");
}
