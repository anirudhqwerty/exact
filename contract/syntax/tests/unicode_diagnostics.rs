use contract_syntax::{Lexer, Span, TokenKind};

fn source(value: &str) -> String {
    format!("component A\n  view\n    text {value}\n")
}

fn unexpected(input: &str) -> String {
    let error = Lexer::tokenize(&source(input), 1).expect_err("bare token must be refused");
    println!(
        "CASE|{input}|{}|{}|{}|{}|{}|{}",
        error.id, error.message, error.span.line, error.span.col,
        error.span.end_col, error.span.source_id
    );
    eprintln!("LexError input={input:?}: {error:?}");
    assert_eq!(error.id, "syntax-unexpected-char");
    assert_eq!(error.span, Span { line: 3, col: 10, end_col: 10, source_id: 0 });
    error.message
}

#[test]
fn control_ascii() {
    assert_eq!(unexpected("@"), "unexpected `@`");
}

#[test]
fn control_quoted_unicode() {
    for input in ["é", "🙂", "—"] {
        let text = source(&format!("\"{input}\""));
        let tokens = Lexer::tokenize(&text, 1).expect("quoted Unicode must lex");
        assert!(tokens.iter().any(|t| matches!(&t.kind, TokenKind::Str(s) if s == input)));
        contract_syntax::parse(&text).expect("quoted Unicode component must parse");
        println!("QUOTED|{input}|lex=ok|parse=ok");
    }
}

fn unicode_message(input: &str) {
    assert_eq!(unexpected(input), format!("unexpected `{input}`"),
        "UNICODE_DIAGNOSTIC_MISMATCH input={input}");
}

#[test]
fn unicode_e() { unicode_message("é"); }

#[test]
fn unicode_emoji() { unicode_message("🙂"); }

#[test]
fn unicode_dash() { unicode_message("—"); }
