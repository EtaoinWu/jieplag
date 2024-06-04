use crate::lang::Tokenize;
use crate::token::Token;
use anyhow::anyhow;
use tree_sitter::Parser;
use tree_sitter_python;

pub struct Python;

impl Tokenize for Python {
    fn tokenize_str(&self, content: &str) -> anyhow::Result<Vec<Token>> {
        tokenize_str(content)
    }
}

#[warn(non_snake_case)]
pub fn tokenize_str(content: &str) -> anyhow::Result<Vec<Token>> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_python::language())
        .expect("Error loading Python grammar");
    let mut res = vec![];
    let tree = parser
        .parse(content, None)
        .ok_or_else(|| anyhow!("Failed to parse the code"))?;
    let root_node = tree.root_node();
    let mut cursor = root_node.walk();
    loop {
        let node = cursor.node();

        'output: {
            if node.child_count() == 0 {
                let kind_str = node.kind();
                let kind: u8 = match kind_str {
                    "comment" => break 'output,
                    "__future__" => 0,
                    "_" => 1,
                    "_compound_statement" => 2,
                    "_simple_statement" => 3,
                    "-" => 4,
                    "-=" => 5,
                    "->" => 6,
                    "," => 7,
                    ";" => 8,
                    ":" => 9,
                    ":=" => 10,
                    "!=" => 11,
                    "." => 12,
                    "(" => 13,
                    ")" => 14,
                    "[" => 15,
                    "]" => 16,
                    "{" => 17,
                    "}" => 18,
                    "@" => 19,
                    "@=" => 20,
                    "*" => 21,
                    "**" => 22,
                    "**=" => 23,
                    "*=" => 24,
                    "/" => 25,
                    "//" => 26,
                    "//=" => 27,
                    "/=" => 28,
                    "&" => 29,
                    "&=" => 30,
                    "%" => 31,
                    "%=" => 32,
                    "^" => 33,
                    "^=" => 34,
                    "+" => 35,
                    "+=" => 36,
                    "<" => 37,
                    "<<" => 38,
                    "<<=" => 39,
                    "<=" => 40,
                    "<>" => 41,
                    "=" => 42,
                    "==" => 43,
                    ">" => 44,
                    ">=" => 45,
                    ">>" => 46,
                    ">>=" => 47,
                    "|" => 48,
                    "|=" => 49,
                    "~" => 50,
                    "aliased_import" => 51,
                    "and" => 52,
                    "argument_list" => 53,
                    "as_pattern_target" => 54,
                    "as_pattern" => 55,
                    "as" => 56,
                    "assert_statement" => 57,
                    "assert" => 58,
                    "assignment" => 59,
                    "async" => 60,
                    "attribute" => 61,
                    "augmented_assignment" => 62,
                    "await" => 63,
                    "binary_operator" => 64,
                    "block" => 65,
                    "boolean_operator" => 66,
                    "break_statement" => 67,
                    "break" => 68,
                    "call" => 69,
                    "case_clause" => 70,
                    "case_pattern" => 71,
                    "case" => 72,
                    "chevron" => 73,
                    "class_definition" => 74,
                    "class_pattern" => 75,
                    "class" => 76,
                    "comparison_operator" => 78,
                    "complex_pattern" => 79,
                    "concatenated_string" => 80,
                    "conditional_expression" => 81,
                    "constrained_type" => 82,
                    "continue_statement" => 83,
                    "continue" => 84,
                    "decorated_definition" => 85,
                    "decorator" => 86,
                    "def" => 87,
                    "default_parameter" => 88,
                    "del" => 89,
                    "delete_statement" => 90,
                    "dict_pattern" => 91,
                    "dictionary_comprehension" => 92,
                    "dictionary_splat_pattern" => 93,
                    "dictionary_splat" => 94,
                    "dictionary" => 95,
                    "dotted_name" => 96,
                    "elif_clause" => 97,
                    "elif" => 98,
                    "ellipsis" => 99,
                    "else_clause" => 100,
                    "else" => 101,
                    "escape_interpolation" => 102,
                    "escape_sequence" => 103,
                    "except_clause" => 104,
                    "except_group_clause" => 105,
                    "except" => 106,
                    "except*" => 107,
                    "exec_statement" => 108,
                    "exec" => 109,
                    "expression_list" => 110,
                    "expression_statement" => 111,
                    "expression" => 112,
                    "false" => 113,
                    "finally_clause" => 114,
                    "finally" => 115,
                    "float" => 116,
                    "for_in_clause" => 117,
                    "for_statement" => 118,
                    "for" => 119,
                    "format_expression" => 120,
                    "format_specifier" => 121,
                    "from" => 122,
                    "function_definition" => 123,
                    "future_import_statement" => 124,
                    "generator_expression" => 125,
                    "generic_type" => 126,
                    "global_statement" => 127,
                    "global" => 128,
                    "identifier" => 129,
                    "if_clause" => 130,
                    "if_statement" => 131,
                    "if" => 132,
                    "import_from_statement" => 133,
                    "import_prefix" => 134,
                    "import_statement" => 135,
                    "import" => 136,
                    "in" => 137,
                    "integer" => 138,
                    "interpolation" => 139,
                    "is not" => 140,
                    "is" => 141,
                    "keyword_argument" => 142,
                    "keyword_pattern" => 143,
                    "keyword_separator" => 144,
                    "lambda_parameters" => 145,
                    "lambda" => 146,
                    "line_continuation" => 147,
                    "list_comprehension" => 148,
                    "list_pattern" => 149,
                    "list_splat_pattern" => 150,
                    "list_splat" => 151,
                    "list" => 152,
                    "match_statement" => 153,
                    "match" => 154,
                    "member_type" => 155,
                    "module" => 156,
                    "named_expression" => 157,
                    "none" => 158,
                    "nonlocal_statement" => 159,
                    "nonlocal" => 160,
                    "not in" => 161,
                    "not_operator" => 162,
                    "not" => 163,
                    "or" => 164,
                    "pair" => 165,
                    "parameter" => 166,
                    "parameters" => 167,
                    "parenthesized_expression" => 168,
                    "parenthesized_list_splat" => 169,
                    "pass_statement" => 170,
                    "pass" => 171,
                    "pattern_list" => 172,
                    "pattern" => 173,
                    "positional_separator" => 174,
                    "primary_expression" => 175,
                    "print_statement" => 176,
                    "print" => 177,
                    "raise_statement" => 178,
                    "raise" => 179,
                    "relative_import" => 180,
                    "return_statement" => 181,
                    "return" => 182,
                    "set_comprehension" => 183,
                    "set" => 184,
                    "slice" => 185,
                    "splat_pattern" => 186,
                    "splat_type" => 187,
                    "string_content" => 188,
                    "string_end" => 189,
                    "string_start" => 190,
                    "string" => 191,
                    "subscript" => 192,
                    "true" => 193,
                    "try_statement" => 194,
                    "try" => 195,
                    "tuple_pattern" => 196,
                    "tuple" => 197,
                    "type_alias_statement" => 198,
                    "type_conversion" => 199,
                    "type_parameter" => 200,
                    "type" => 201,
                    "typed_default_parameter" => 202,
                    "typed_parameter" => 203,
                    "unary_operator" => 204,
                    "union_pattern" => 205,
                    "union_type" => 206,
                    "while_statement" => 207,
                    "while" => 208,
                    "wildcard_import" => 209,
                    "with_clause" => 210,
                    "with_item" => 211,
                    "with_statement" => 212,
                    "with" => 213,
                    "yield" => 214,
                    _ => 215,
                };
                let text = &content[node.byte_range()];
                let start_position = node.start_position();
                res.push(Token {
                    kind,
                    spelling: text.to_string(),
                    line: (start_position.row + 1) as u32,
                    column: (start_position.column + 1) as u32,
                });
            }
        }

        if cursor.goto_first_child() {
            continue;
        }

        if cursor.goto_next_sibling() {
            continue;
        }

        while cursor.goto_parent() && !cursor.goto_next_sibling() {}

        if cursor.node() == tree.root_node() {
            break;
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::tokenize_str;

    #[test]
    fn test_tokenize() {
        let code = "a = input() #42\nb = input()\nprint(a+b)";
        let tokens = tokenize_str(code).unwrap();

        eprintln!("{:?}", tokens);

        assert_eq!(tokens[0].spelling, "a");
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);

        assert_eq!(tokens[1].spelling, "=");
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 3);

        assert_eq!(tokens[2].spelling, "input");
        assert_eq!(tokens[2].line, 1);
        assert_eq!(tokens[2].column, 5);

        assert_eq!(tokens[13].spelling, "+");
        assert_eq!(tokens[13].line, 3);
        assert_eq!(tokens[13].column, 8);
    }
}
