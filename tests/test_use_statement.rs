use hoditor::ts_highlighter::{TokenType, TreeSitterHighlighter};

#[test]
fn test_use_statement_colors() {
    let mut highlighter = TreeSitterHighlighter::new();

    let test_cases = vec![
        ("use crate::app::Context;", "use", TokenType::Keyword),
        ("use crate::app::Context;", "Context", TokenType::Type),
        ("use std::fs::File;", "File", TokenType::Type),
        (
            "use command_dispatcher::CmdDispatcher;",
            "CmdDispatcher",
            TokenType::Type,
        ),
        // Also test that enum variants in normal code are still Constants
        (
            "let x = Command::DoNothing;",
            "DoNothing",
            TokenType::Constant,
        ),
    ];

    println!("\n{:=<100}", "");
    println!("TESTING USE STATEMENT HIGHLIGHTING");
    println!("{:=<100}", "");

    for (line, token_name, expected_type) in test_cases {
        println!("\nLine: {}", line);
        let tokens = highlighter.highlight_line(line);

        println!("  All tokens:");
        for (start, end, token_type) in &tokens {
            let text = &line[*start..*end];
            let color = token_type.to_color();
            if let crossterm::style::Color::Rgb { r, g, b } = color {
                println!(
                    "    '{}' => {:?} (RGB: #{:02x}{:02x}{:02x})",
                    text, token_type, r, g, b
                );
            }
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
            panic!("{} was not highlighted at all in: {}", token_name, line);
        }
    }

    println!("\n{:=<100}", "");
}
