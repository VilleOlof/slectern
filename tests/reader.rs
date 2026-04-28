use slectern::{Reader, ReaderError};

#[test]
fn skip_whitespace_none() -> Result<(), ReaderError> {
    let mut r = Reader::new("Hello!");
    r.skip_whitespace();
    assert_eq!(r.pos(), 0);
    Ok(())
}

#[test]
fn skip_whitespace_mixed() -> Result<(), ReaderError> {
    let mut r = Reader::new(" \t \t\nHello!");
    r.skip_whitespace();
    assert_eq!(r.pos(), 5);
    Ok(())
}

#[test]
fn peek() -> Result<(), ReaderError> {
    let r = Reader::new("efwhnio");
    assert_eq!(r.peek(0), 'e');
    assert_eq!(r.peek(4), 'n');
    assert_eq!(r.peek(10), 'o');
    Ok(())
}

#[test]
fn peek_n() -> Result<(), ReaderError> {
    let mut r = Reader::new("efwhnio");
    assert_eq!(r.peek_n(1), "e");
    assert_eq!(r.peek_n(4), "efwh");
    r.skip();
    r.skip();
    assert_eq!(r.peek_n(481), "whnio");
    Ok(())
}

#[test]
fn read_unquoted() -> Result<(), ReaderError> {
    let mut r = Reader::new("hello world");
    assert_eq!(r.read_unquoted_string()?, "hello");
    assert_eq!(r.prev(), "hello");
    assert_eq!(r.next(), " world");
    Ok(())
}

#[test]
fn read_unquoted_empty() -> Result<(), ReaderError> {
    let mut r = Reader::new("");
    assert_eq!(r.read_unquoted_string()?, "");
    assert_eq!(r.prev(), "");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_quoted() -> Result<(), ReaderError> {
    let mut r = Reader::new("\"hello world\"");
    assert_eq!(r.read_quoted_string()?, "hello world");
    assert_eq!(r.prev(), "\"hello world\"");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_single_quoted() -> Result<(), ReaderError> {
    let mut r = Reader::new("'hello world'");
    assert_eq!(r.read_quoted_string()?, "hello world");
    assert_eq!(r.prev(), "'hello world'");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_mixed_quoted_double_inside_single() -> Result<(), ReaderError> {
    let mut r = Reader::new("'hello \"world\"'");
    assert_eq!(r.read_quoted_string()?, "hello \"world\"");
    assert_eq!(r.prev(), "'hello \"world\"'");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_quoted_empty() -> Result<(), ReaderError> {
    let mut r = Reader::new("\"\"");
    assert_eq!(r.read_quoted_string()?, "");
    assert_eq!(r.prev(), "\"\"");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_n() -> Result<(), ReaderError> {
    let mut r = Reader::new("abcdefg");
    assert_eq!(r.read_n(3)?, "abc");
    assert_eq!(r.prev(), "abc");
    assert_eq!(r.next(), "defg");
    Ok(())
}

#[test]
fn read_until_vec() -> Result<(), ReaderError> {
    let mut r = Reader::new("name[]");
    assert_eq!(r.read_string_until_vec(&vec![' ', '['])?, "name");
    assert_eq!(r.prev(), "name");
    assert_eq!(r.next(), "[]");
    Ok(())
}

#[test]
fn read_i32() -> Result<(), ReaderError> {
    let mut r = Reader::new("1234567890");
    assert_eq!(r.read_num::<i32>()?, 1234567890);
    assert_eq!(r.prev(), "1234567890");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_negative_i32() -> Result<(), ReaderError> {
    let mut r = Reader::new("-1234567890");
    assert_eq!(r.read_num::<i32>()?, -1234567890);
    assert_eq!(r.prev(), "-1234567890");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_invalid_i32() -> Result<(), ReaderError> {
    let mut r = Reader::new("532.13");
    assert!(r.read_num::<i32>().is_err());
    Ok(())
}

#[test]
fn read_empty_i32() -> Result<(), ReaderError> {
    let mut r = Reader::new("");
    assert!(r.read_num::<i32>().is_err());
    Ok(())
}

#[test]
fn read_i64() -> Result<(), ReaderError> {
    let mut r = Reader::new("123456789012345678");
    assert_eq!(r.read_num::<i64>()?, 123456789012345678);
    assert_eq!(r.prev(), "123456789012345678");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_negative_i64() -> Result<(), ReaderError> {
    let mut r = Reader::new("-123456789012345678");
    assert_eq!(r.read_num::<i64>()?, -123456789012345678);
    assert_eq!(r.prev(), "-123456789012345678");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_f64_with_decimal() -> Result<(), ReaderError> {
    let mut r = Reader::new("513.1956016581");
    assert_eq!(r.read_num::<f64>()?, 513.1956016581);
    assert_eq!(r.prev(), "513.1956016581");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_f64() -> Result<(), ReaderError> {
    let mut r = Reader::new("513");
    assert_eq!(r.read_num::<f64>()?, 513.);
    assert_eq!(r.prev(), "513");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_negative_f64() -> Result<(), ReaderError> {
    let mut r = Reader::new("-1285.311");
    assert_eq!(r.read_num::<f64>()?, -1285.311);
    assert_eq!(r.prev(), "-1285.311");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_spaced_num() -> Result<(), ReaderError> {
    let mut r = Reader::new(" 58281911 ");
    r.skip_whitespace();
    assert_eq!(r.read_num::<i32>()?, 58281911);
    assert_eq!(r.prev(), " 58281911");
    assert_eq!(r.next(), " ");
    Ok(())
}

#[test]
fn expect_correct() -> Result<(), ReaderError> {
    let mut r = Reader::new("abc");
    r.expect('a')?;
    assert_eq!(r.pos(), 1);
    Ok(())
}

#[test]
fn expect_incorrect() -> Result<(), ReaderError> {
    let mut r = Reader::new("bcd");
    assert!(r.expect('a').is_err());
    Ok(())
}

#[test]
fn read_bracket() -> Result<(), ReaderError> {
    let mut r = Reader::new("[[], hello, world]");
    assert_eq!(r.read_until_balanced(('[', ']'))?, "[[], hello, world]");
    assert_eq!(r.prev(), "[[], hello, world]");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_broken_bracket() -> Result<(), ReaderError> {
    let mut r = Reader::new("[5, 1, []");
    assert!(r.read_until_balanced(('[', ']')).is_err());
    Ok(())
}

#[test]
fn read_open_bracket() -> Result<(), ReaderError> {
    let mut r = Reader::new("[hello! and more");
    assert!(r.read_until_balanced(('[', ']')).is_err());
    Ok(())
}

#[test]
fn read_whitespaced_bracket() -> Result<(), ReaderError> {
    let mut r = Reader::new(" [stuff inside] ");
    assert_eq!(r.read_until_balanced(('[', ']'))?, " [stuff inside]");
    assert_eq!(r.prev(), " [stuff inside]");
    assert_eq!(r.next(), " ");
    Ok(())
}

#[test]
fn read_pre_bracket() -> Result<(), ReaderError> {
    let mut r = Reader::new("name[value={}, nested: [], a: true] 52.13 other");
    assert_eq!(
        r.read_until_balanced(('[', ']'))?,
        "name[value={}, nested: [], a: true]"
    );
    assert_eq!(r.prev(), "name[value={}, nested: [], a: true]");
    assert_eq!(r.next(), " 52.13 other");
    Ok(())
}

#[test]
fn read_quoted_bracket() -> Result<(), ReaderError> {
    let mut r = Reader::new("[\"key\": 913, \"bracketkey]\": 9]");
    assert_eq!(
        r.read_until_balanced(('[', ']'))?,
        "[\"key\": 913, \"bracketkey]\": 9]"
    );
    assert_eq!(r.prev(), "[\"key\": 913, \"bracketkey]\": 9]");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn read_only_close_bracket() -> Result<(), ReaderError> {
    let mut r = Reader::new("hello]");
    assert!(r.read_until_balanced(('[', ']')).is_err());
    Ok(())
}

#[test]
fn read_only_open_quote_in_bracket() -> Result<(), ReaderError> {
    let mut r = Reader::new("[5, 1, ', 8]");
    assert!(r.read_until_balanced(('[', ']')).is_err());
    Ok(())
}

#[test]
fn read_bool() -> Result<(), ReaderError> {
    let mut r = Reader::new("true");
    assert_eq!(r.read_bool()?, true);
    assert_eq!(r.prev(), "true");
    assert_eq!(r.next(), "");
    Ok(())
}

#[test]
fn test_command() -> Result<(), ReaderError> {
    let mut r =
        Reader::new("    fill ~5 ~5 ~5 ~ ~ ~-10 minecraft:oak_log[facing=south] replace air");
    r.skip_whitespace();
    let _fill = r.read_string_until_end()?;
    let _1_x = r.read_string_until_end()?;
    let _1_y = r.read_string_until_end()?;
    let _1_z = r.read_string_until_end()?;

    let _2_x = r.read_string_until_end()?;
    let _2_y = r.read_string_until_end()?;
    let _2_z = r.read_string_until_end()?;

    let _1_block = r.read_string_until_vec(&vec![' ', '['])?;
    let _1_block_state = if r.peek(0) == '[' {
        r.read_until_balanced(('[', ']'))?
    } else {
        String::new()
    };
    r.skip();
    let _modifier = r.read_string_until_end()?;
    let _2_block = r.read_string_until_vec(&vec![' ', '['])?;
    let _2_block_state = if r.peek(0) == '[' {
        r.read_until_balanced(('[', ']'))?
    } else {
        String::new()
    };

    Ok(())
}
