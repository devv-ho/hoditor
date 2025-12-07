use hoditor::ts_highlighter::{TokenType, TreeSitterHighlighter};

#[test]
fn test_new_vs_movemode() {
    let mut highlighter = TreeSitterHighlighter::new();

    // Test the exact pattern the user mentioned
    let line1 = "let mut dispatcher = CmdDispatcher::new();";
    let line2 = "dispatcher.register(\"h\", Command::MoveCursor { dx: -1, dy: 0 });";

    println!("\n{:=<100}", "");
    println!("Testing: {}", line1);
    println!("{:=<100}", "");

    let tokens1 = highlighter.highlight_line(line1);
    for (start, end, token_type) in &tokens1 {
        let text = &line1[*start..*end];
        let color = token_type.to_color();
        if let crossterm::style::Color::Rgb { r, g, b } = color {
            println!(
                "  '{}' => {:?} (RGB: #{:02x}{:02x}{:02x})",
                text, token_type, r, g, b
            );
        }
    }

    // Verify 'new' is Function (blue)
    let new_token = tokens1.iter().find(|(s, e, _)| &line1[*s..*e] == "new");
    assert!(new_token.is_some(), "new should be highlighted");
    assert_eq!(
        new_token.unwrap().2,
        TokenType::Function,
        "new should be Function (blue)"
    );
    println!("\n✅ 'new' is Function (Blue #7aa2f7)");

    println!("\n{:=<100}", "");
    println!("Testing: {}", line2);
    println!("{:=<100}", "");

    let tokens2 = highlighter.highlight_line(line2);
    for (start, end, token_type) in &tokens2 {
        let text = &line2[*start..*end];
        let color = token_type.to_color();
        if let crossterm::style::Color::Rgb { r, g, b } = color {
            println!(
                "  '{}' => {:?} (RGB: #{:02x}{:02x}{:02x})",
                text, token_type, r, g, b
            );
        }
    }

    // Verify 'MoveCursor' is Constant (orange)
    let movecursor_token = tokens2
        .iter()
        .find(|(s, e, _)| &line2[*s..*e] == "MoveCursor");
    assert!(
        movecursor_token.is_some(),
        "MoveCursor should be highlighted"
    );
    assert_eq!(
        movecursor_token.unwrap().2,
        TokenType::Constant,
        "MoveCursor should be Constant (orange)"
    );
    println!("\n✅ 'MoveCursor' is Constant (Orange #ff9e64)");

    println!("\n{:=<100}", "");
    println!("RESULT: 'new' (blue) and 'MoveCursor' (orange) now have DIFFERENT colors!");
    println!("{:=<100}", "");
}
