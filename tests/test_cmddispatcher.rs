use hoditor::ts_highlighter::{TokenType, TreeSitterHighlighter};

#[test]
fn test_cmddispatcher_in_struct() {
    let mut highlighter = TreeSitterHighlighter::new();

    // Test with full struct context
    let line = "struct EventHandler { dispatcher: CmdDispatcher }";

    println!("\n{:=<100}", "");
    println!("Testing: {}", line);
    println!("{:=<100}", "");

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

    // Check if CmdDispatcher is captured
    let cmd_token = tokens
        .iter()
        .find(|(s, e, _)| &line[*s..*e] == "CmdDispatcher");

    if let Some((_, _, token_type)) = cmd_token {
        println!("\n  'CmdDispatcher' highlighted as: {:?}", token_type);
        assert_eq!(
            *token_type,
            TokenType::Type,
            "CmdDispatcher should be Type (cyan)"
        );
        println!("  ✅ 'CmdDispatcher' is Type (Cyan #2ac3de)");
    } else {
        println!("\n  ❌ 'CmdDispatcher' was NOT highlighted at all!");
        panic!("CmdDispatcher should be highlighted as Type");
    }

    println!("{:=<100}", "");
}
