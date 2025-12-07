use hoditor::ts_highlighter::{TokenType, TreeSitterHighlighter};

#[test]
fn test_all_enum_variant_patterns() {
    let mut highlighter = TreeSitterHighlighter::new();

    let test_cases = vec![
        (
            "Command::MoveCursor { dx: 1, dy: 0 }",
            "MoveCursor",
            TokenType::Constant,
        ),
        (
            "Command::ChangeMode(Mode::Edit)",
            "ChangeMode",
            TokenType::Constant,
        ),
        (
            "let x = Command::DoNothing;",
            "DoNothing",
            TokenType::Constant,
        ),
        (
            "dispatcher.register(\"w\", Command::Save);",
            "Save",
            TokenType::Constant,
        ),
        (
            "dispatcher.register(\"gg\", Command::MoveCursorSOF);",
            "MoveCursorSOF",
            TokenType::Constant,
        ),
        (
            "Command::ChangeMode(Mode::Edit)",
            "Edit",
            TokenType::Constant,
        ),
    ];

    println!("\n{:=<100}", "");
    println!("TESTING ENUM VARIANT HIGHLIGHTING");
    println!("{:=<100}", "");

    for (line, variant_name, expected_type) in test_cases {
        println!("\nLine: {}", line);
        let tokens = highlighter.highlight_line(line);

        println!("  All tokens:");
        for (start, end, token_type) in &tokens {
            let text = &line[*start..*end];
            println!("    '{}' => {:?}", text, token_type);
        }

        // Also check if Command is being captured
        let command_token = tokens.iter().find(|(start, end, _)| {
            let text = &line[*start..*end];
            text == "Command" || text.starts_with("Command")
        });
        if let Some((start, end, tt)) = command_token {
            println!(
                "  DEBUG: Found 'Command' token: '{}' => {:?}",
                &line[*start..*end],
                tt
            );
        } else {
            println!("  DEBUG: 'Command' not found in tokens!");
        }

        let variant_token = tokens
            .iter()
            .find(|(start, end, _)| &line[*start..*end] == variant_name);

        if let Some((_, _, token_type)) = variant_token {
            println!(
                "  '{}' highlighted as: {:?} (expected: {:?})",
                variant_name, token_type, expected_type
            );
            assert_eq!(
                *token_type, expected_type,
                "{} should be {:?} but was {:?}",
                variant_name, expected_type, token_type
            );
        } else {
            panic!("{} was not highlighted at all!", variant_name);
        }
    }

    println!("\n{:=<100}", "");
}
