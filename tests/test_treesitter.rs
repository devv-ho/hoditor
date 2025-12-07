use hoditor::ts_highlighter::TreeSitterHighlighter;

#[test]
fn test_treesitter_enum_variants() {
    let mut highlighter = TreeSitterHighlighter::new();

    let test_cases = vec![
        (
            "dispatcher.register(\"h\", Command::MoveCursor { dx: -1, dy: 0 });",
            vec!["Command", "MoveCursor"],
        ),
        (
            "enum Command { DoNothing, MoveCursor { dx: i32 } }",
            vec!["Command", "DoNothing", "MoveCursor"],
        ),
        ("use std::collections::HashMap;", vec!["HashMap"]),
        (
            "impl Command { fn execute() {} }",
            vec!["Command", "execute"],
        ),
    ];

    println!("\n{:=<100}", "");
    println!("TREESITTER HIGHLIGHTING TEST");
    println!("{:=<100}", "");

    for (line, expected_tokens) in test_cases {
        println!("\nLine: {}", line);
        let tokens = highlighter.highlight_line(line);

        let highlighted_texts: Vec<String> = tokens
            .iter()
            .map(|(start, end, _)| line[*start..*end].to_string())
            .collect();

        println!("  Highlighted tokens: {:?}", highlighted_texts);

        for expected in expected_tokens {
            let found = highlighted_texts.iter().any(|t| t == expected);
            println!(
                "  {} is highlighted: {}",
                expected,
                if found { "✓" } else { "✗" }
            );
            assert!(found, "{} should be highlighted but wasn't", expected);
        }
    }

    println!("\n{:=<100}", "");
}
