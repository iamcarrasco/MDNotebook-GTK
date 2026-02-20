/// Shared markdown block-detection logic used by both `wysiwyg` and `preview`.

#[derive(Debug)]
pub enum BlockKind<'a> {
    HorizontalRule,
    Heading1(&'a str),
    Heading2(&'a str),
    Heading3(&'a str),
    Heading4(&'a str),
    Heading5(&'a str),
    Heading6(&'a str),
    Quote(&'a str),
    TaskUnchecked(&'a str),
    TaskChecked(&'a str),
    OrderedList {
        number: &'a str,
        rest: &'a str,
        prefix_chars: usize,
    },
    BulletList(&'a str),
    Plain(&'a str),
}

pub fn detect_block(line: &str) -> BlockKind<'_> {
    let trimmed = line.trim();
    if is_horizontal_rule(trimmed) {
        return BlockKind::HorizontalRule;
    }
    if let Some(rest) = line.strip_prefix("###### ") {
        return BlockKind::Heading6(rest);
    }
    if let Some(rest) = line.strip_prefix("##### ") {
        return BlockKind::Heading5(rest);
    }
    if let Some(rest) = line.strip_prefix("#### ") {
        return BlockKind::Heading4(rest);
    }
    if let Some(rest) = line.strip_prefix("### ") {
        return BlockKind::Heading3(rest);
    }
    if let Some(rest) = line.strip_prefix("## ") {
        return BlockKind::Heading2(rest);
    }
    if let Some(rest) = line.strip_prefix("# ") {
        return BlockKind::Heading1(rest);
    }
    if let Some(rest) = line.strip_prefix("> ") {
        return BlockKind::Quote(rest);
    }
    if let Some(rest) = line
        .strip_prefix("- [ ] ")
        .or_else(|| line.strip_prefix("* [ ] "))
    {
        return BlockKind::TaskUnchecked(rest);
    }
    if let Some(rest) = line
        .strip_prefix("- [x] ")
        .or_else(|| line.strip_prefix("* [x] "))
        .or_else(|| line.strip_prefix("- [X] "))
        .or_else(|| line.strip_prefix("* [X] "))
    {
        return BlockKind::TaskChecked(rest);
    }
    if let Some((number, rest, prefix_chars)) = parse_ordered_prefix(line) {
        return BlockKind::OrderedList {
            number,
            rest,
            prefix_chars,
        };
    }
    if let Some(rest) = line
        .strip_prefix("- ")
        .or_else(|| line.strip_prefix("* "))
        .or_else(|| line.strip_prefix("\u{2022} "))
    {
        return BlockKind::BulletList(rest);
    }
    BlockKind::Plain(line)
}

pub fn is_horizontal_rule(trimmed: &str) -> bool {
    if trimmed.len() < 3 {
        return false;
    }
    let dash_count = trimmed.chars().filter(|&c| c == '-').count();
    let star_count = trimmed.chars().filter(|&c| c == '*').count();
    let underscore_count = trimmed.chars().filter(|&c| c == '_').count();

    (dash_count >= 3 && trimmed.chars().all(|c| c == '-' || c.is_whitespace()))
        || (star_count >= 3 && trimmed.chars().all(|c| c == '*' || c.is_whitespace()))
        || (underscore_count >= 3 && trimmed.chars().all(|c| c == '_' || c.is_whitespace()))
}

pub fn parse_ordered_prefix(text: &str) -> Option<(&str, &str, usize)> {
    let bytes = text.as_bytes();
    let mut index = 0;

    while index < bytes.len() && bytes[index].is_ascii_digit() {
        index += 1;
    }

    if index == 0 || index + 1 >= bytes.len() {
        return None;
    }

    if (bytes[index] != b'.' && bytes[index] != b')') || bytes[index + 1] != b' ' {
        return None;
    }

    let number = &text[..index];
    let rest = &text[(index + 2)..];
    Some((number, rest, index + 2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading1() {
        assert!(matches!(detect_block("# Hello"), BlockKind::Heading1("Hello")));
    }

    #[test]
    fn heading2() {
        assert!(matches!(detect_block("## Sub"), BlockKind::Heading2("Sub")));
    }

    #[test]
    fn heading3() {
        assert!(matches!(detect_block("### Third"), BlockKind::Heading3("Third")));
    }

    #[test]
    fn heading4() {
        assert!(matches!(detect_block("#### Fourth"), BlockKind::Heading4("Fourth")));
    }

    #[test]
    fn heading5() {
        assert!(matches!(detect_block("##### Fifth"), BlockKind::Heading5("Fifth")));
    }

    #[test]
    fn heading6() {
        assert!(matches!(detect_block("###### Sixth"), BlockKind::Heading6("Sixth")));
    }

    #[test]
    fn quote() {
        assert!(matches!(detect_block("> text"), BlockKind::Quote("text")));
    }

    #[test]
    fn bullet_dash() {
        assert!(matches!(detect_block("- item"), BlockKind::BulletList("item")));
    }

    #[test]
    fn bullet_star() {
        assert!(matches!(detect_block("* item"), BlockKind::BulletList("item")));
    }

    #[test]
    fn ordered_list() {
        match detect_block("1. first") {
            BlockKind::OrderedList { number, rest, prefix_chars } => {
                assert_eq!(number, "1");
                assert_eq!(rest, "first");
                assert_eq!(prefix_chars, 3);
            }
            other => panic!("Expected OrderedList, got {:?}", other),
        }
    }

    #[test]
    fn ordered_list_multidigit() {
        match detect_block("42. answer") {
            BlockKind::OrderedList { number, rest, .. } => {
                assert_eq!(number, "42");
                assert_eq!(rest, "answer");
            }
            other => panic!("Expected OrderedList, got {:?}", other),
        }
    }

    #[test]
    fn task_unchecked() {
        assert!(matches!(detect_block("- [ ] todo"), BlockKind::TaskUnchecked("todo")));
    }

    #[test]
    fn task_checked() {
        assert!(matches!(detect_block("- [x] done"), BlockKind::TaskChecked("done")));
    }

    #[test]
    fn task_checked_capital() {
        assert!(matches!(detect_block("- [X] done"), BlockKind::TaskChecked("done")));
    }

    #[test]
    fn horizontal_rule_dashes() {
        assert!(matches!(detect_block("---"), BlockKind::HorizontalRule));
        assert!(matches!(detect_block("------"), BlockKind::HorizontalRule));
        assert!(matches!(detect_block("- - -"), BlockKind::HorizontalRule));
    }

    #[test]
    fn horizontal_rule_stars() {
        assert!(matches!(detect_block("***"), BlockKind::HorizontalRule));
    }

    #[test]
    fn horizontal_rule_underscores() {
        assert!(matches!(detect_block("___"), BlockKind::HorizontalRule));
    }

    #[test]
    fn plain_text() {
        assert!(matches!(detect_block("just text"), BlockKind::Plain("just text")));
    }

    #[test]
    fn ordered_prefix_valid() {
        assert_eq!(parse_ordered_prefix("1. hello"), Some(("1", "hello", 3)));
    }

    #[test]
    fn ordered_prefix_no_space() {
        assert!(parse_ordered_prefix("1.hello").is_none());
    }

    #[test]
    fn ordered_prefix_no_dot() {
        assert!(parse_ordered_prefix("1 hello").is_none());
    }

    #[test]
    fn ordered_list_paren() {
        match detect_block("1) first") {
            BlockKind::OrderedList { number, rest, prefix_chars } => {
                assert_eq!(number, "1");
                assert_eq!(rest, "first");
                assert_eq!(prefix_chars, 3);
            }
            other => panic!("Expected OrderedList, got {:?}", other),
        }
    }

    #[test]
    fn ordered_prefix_paren() {
        assert_eq!(parse_ordered_prefix("1) hello"), Some(("1", "hello", 3)));
    }

    #[test]
    fn horizontal_rule_too_short() {
        assert!(!is_horizontal_rule("--"));
        assert!(!is_horizontal_rule(""));
    }
}
