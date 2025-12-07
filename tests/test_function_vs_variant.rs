use hoditor::ts_highlighter::{TokenType, TreeSitterHighlighter};

#[test]
fn test_function_vs_enum_variant_calls() {
    let mut highlighter = TreeSitterHighlighter::new();

    let test_cases = vec![
        // Associated function calls (should be Function, blue)
        ("CmdDispatcher::new()", "new", TokenType::Function),
        ("HashMap::new()", "new", TokenType::Function),
        // Enum variant calls (should be Constant, orange)
        (
            "Command::ChangeMode(Mode::Edit)",
            "ChangeMode",
            TokenType::Constant,
        ),
        ("Option::Some(42)", "Some", TokenType::Constant),
        // The type names should all be Type
        ("CmdDispatcher::new()", "CmdDispatcher", TokenType::Type),
        (
            "Command::ChangeMode(Mode::Edit)",
            "Command",
            TokenType::Type,
        ),
    ];

    println!("\n{:=<100}", "");
    println!("TESTING FUNCTION vs ENUM VARIANT CALLS");
    println!("{:=<100}", "");

    for (line, token_name, expected_type) in test_cases {
        println!("\nLine: {}", line);
        let tokens = highlighter.highlight_line(line);

        println!("  All tokens:");
        for (start, end, token_type) in &tokens {
            let text = &line[*start..*end];
            println!("    '{}' => {:?}", text, token_type);
        }

        let token = tokens
            .iter()
            .find(|(start, end, _)| &line[*start..*end] == token_name);

        if let Some((_, _, token_type)) = token {
            println!(
                "  '{}' highlighted as: {:?} (expected: {:?})",
                token_name, token_type, expected_type
            );
            assert_eq!(
                *token_type, expected_type,
                "{} should be {:?} but was {:?}",
                token_name, expected_type, token_type
            );
        } else {
            panic!("{} was not highlighted at all!", token_name);
        }
    }

    println!("\n{:=<100}", "");
}
