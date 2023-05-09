use super::*;

const HELLO_WORLD: &[u8] = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn exec_helper(bf: &[u8], mut reader: &[u8]) -> Vec<u8> {
    let instructions = parse(bf);
    let mut writer = Vec::new();
    exec(&instructions, &mut reader, &mut writer);
    writer
}

#[test]
fn test_hello_world() {
    assert_eq!(exec_helper(HELLO_WORLD, b""), b"Hello World!\n");
}

#[test]
fn test_empty() {
    assert_eq!(exec_helper(b"", b""), b"");
}

#[test]
fn test_read() {
    assert_eq!(exec_helper(b",.,.", b"ab"), b"ab");
}

#[test]
#[should_panic(expected = "unclosed loop")]
fn unclosed_loop() {
    exec_helper(b"[", b"");
}

#[test]
#[should_panic(expected = "unclosed loop")]
fn unclosed_loop_nonzero() {
    exec_helper(b"+[", b"");
}

#[test]
#[should_panic(expected = "unexpected closing bracket")]
fn unexpected_closing_bracket() {
    exec_helper(b"]", b"");
}
