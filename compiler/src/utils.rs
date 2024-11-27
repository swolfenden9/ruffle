pub fn rows_cols_index(input: &str, index: usize) -> (usize, usize) {
    let mut rows = 1;
    let mut cols = 1;

    for (i, char) in input.chars().enumerate() {
        if i == index {
            break;
        }

        if char == '\n' {
            rows += 1;
            cols = 1;
        } else {
            cols += 1;
        }
    }

    (rows, cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line() {
        assert_eq!(rows_cols_index("Hello, world!", 0), (1, 1));
        assert_eq!(rows_cols_index("Hello, world!", 6), (1, 7));
        assert_eq!(rows_cols_index("Hello, world!", 12), (1, 13));
    }

    #[test]
    fn test_multiple_lines() {
        let input = "Hello\nWorld\nRust!";
        assert_eq!(rows_cols_index(input, 0), (1, 1)); // Start of first line
        assert_eq!(rows_cols_index(input, 5), (1, 6)); // End of first line
        assert_eq!(rows_cols_index(input, 6), (2, 1)); // Start of second line
        assert_eq!(rows_cols_index(input, 10), (2, 5)); // Middle of second line
        assert_eq!(rows_cols_index(input, 12), (3, 1)); // Start of third line
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(rows_cols_index("", 0), (1, 1)); // Index in empty string
    }

    #[test]
    fn test_out_of_bounds() {
        assert_eq!(rows_cols_index("Hello", 100), (1, 6)); // Beyond end of string
    }

    #[test]
    fn test_boundary_conditions() {
        let input = "Hello\n";
        assert_eq!(rows_cols_index(input, 5), (1, 6)); // At newline boundary
        assert_eq!(rows_cols_index(input, 6), (2, 1)); // After newline
    }
}
