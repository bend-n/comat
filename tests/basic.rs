use comat::comat;
#[test]
fn basic() {
    assert_eq!(comat!("{red}yes{reset}"), "\x1b[0;34;31myes\x1b[0m");
    assert_eq!(comat!("{thing:red}"), "\x1b[0m\x1b[0;34;31m{thing}\x1b[0m");
    assert_eq!(comat!("{n:.0}"), "{n:.0}");
}

#[test]
fn escapes() {
    assert_eq!(comat!("{{ow}} {{red}}"), "{ow} {red}");
    assert_eq!(comat!("{{{{"), "{{");
}

#[test]
fn take() {
    assert_eq!(comat!("{}"), "{}");
}

#[test]
fn resetty() {
    assert_eq!(comat!("{:reset}"), "\x1b[0m{}\x1b[0m");
}
