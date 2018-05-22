mod tests {
  use ::main;


  #[test]
  fn empty_contents_returns_empty_vec() {
    let result = main::execute("", "").unwrap();
    let expected: Vec<String> = vec![];
    assert_eq!(result, expected);
  }

  #[test]
  fn simple_pattern_with_no_match_returns_empty_vec() {
    let result = main::execute("corpus\n", "e").unwrap();
    let expected: Vec<String> = vec![];
    assert_eq!(result, expected);
  }

  #[test]
  fn simple_pattern_with_match_returns_vec_with_str_colored_on_match() {
    let result = main::execute("corpus\n", "u").unwrap();
    let expected: Vec<String> = vec![String::from("corp\u{1b}[31mu\u{1b}[0ms")];
    assert_eq!(result, expected);
  }

  #[test]
  fn simple_pattern_with_multiple_matches_returns_vec_with_multiple_colored_str() {
    let result = main::execute("corpus\n", "[ou]").unwrap();
    let expected: Vec<String> = vec![String::from("c\u{1b}[31mo\u{1b}[0mrp\u{1b}[31mu\u{1b}[0ms")];
    assert_eq!(result, expected);
  }
}
