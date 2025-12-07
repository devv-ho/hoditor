use hoditor::ts_highlighter::{TokenType, TreeSitterHighlighter};

#[test]
fn test_struct_definition_colors() {
    let mut highlighter = TreeSitterHighlighter::new();

    let line = "pub struct EventHandler {";

    println!("\n{:=<100}", "");
    println!("Testing: {}", line);
    println!("{:=<100}", "");

    let tokens = highlighter.highlight_line(line);

    for (start, end, token_type) in &tokens {
        let text = &line[*start..*end];
        let color = token_type.to_color();
        if let crossterm::style::Color::Rgb { r, g, b } = color {
            println!(
                "  '{}' => {:?} (RGB: #{:02x}{:02x}{:02x})",
                text, token_type, r, g, b
            );
        }
    }

    // Check 'pub' is Keyword (purple)
    let pub_token = tokens.iter().find(|(s, e, _)| &line[*s..*e] == "pub");
    assert!(pub_token.is_some(), "pub should be highlighted");
    assert_eq!(pub_token.unwrap().2, TokenType::Keyword);
    println!("\n✅ 'pub' is Keyword (Purple #bb9af7)");

    // Check 'struct' is Keyword (purple)
    let struct_token = tokens.iter().find(|(s, e, _)| &line[*s..*e] == "struct");
    assert!(struct_token.is_some(), "struct should be highlighted");
    assert_eq!(struct_token.unwrap().2, TokenType::Keyword);
    println!("✅ 'struct' is Keyword (Purple #bb9af7)");

    // Check 'EventHandler' is Type (cyan)
    let type_token = tokens
        .iter()
        .find(|(s, e, _)| &line[*s..*e] == "EventHandler");
    assert!(type_token.is_some(), "EventHandler should be highlighted");
    assert_eq!(type_token.unwrap().2, TokenType::Type);
    println!("✅ 'EventHandler' is Type (Cyan #2ac3de)");

    println!("\n{:=<100}", "");

    // Also test impl
    let line2 = "impl EventHandler {";
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

    let impl_token = tokens2.iter().find(|(s, e, _)| &line2[*s..*e] == "impl");
    assert!(impl_token.is_some(), "impl should be highlighted");
    assert_eq!(impl_token.unwrap().2, TokenType::Keyword);
    println!("\n✅ 'impl' is Keyword (Purple #bb9af7)");

    println!("{:=<100}", "");
}
