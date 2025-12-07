use syntect::parsing::SyntaxSet;

#[test]
fn list_available_syntaxes() {
    let syntax_set = SyntaxSet::load_defaults_newlines();

    println!("\n{:=<100}", "");
    println!("AVAILABLE SYNTAX DEFINITIONS");
    println!("{:=<100}", "");

    for syntax in syntax_set.syntaxes() {
        if syntax.name.to_lowercase().contains("rust") {
            println!("\nSyntax: {}", syntax.name);
            println!("  File types: {:?}", syntax.file_extensions);
            println!("  Scope: {}", syntax.scope);
            println!("  Variables: {:?}", syntax.variables);
        }
    }

    println!("\n{:=<100}", "");
}

#[test]
fn check_rust_syntax_version() {
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let rust_syntax = syntax_set.find_syntax_by_extension("rs").unwrap();

    println!("\n{:=<100}", "");
    println!("RUST SYNTAX INFORMATION");
    println!("{:=<100}", "");
    println!("Name: {}", rust_syntax.name);
    println!("Scope: {}", rust_syntax.scope);
    println!("Hidden: {}", rust_syntax.hidden);
    println!("Variables: {:#?}", rust_syntax.variables);
    println!("\n{:=<100}", "");
}
