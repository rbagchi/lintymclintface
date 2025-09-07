use crate::{LinterError, SyntaxError};
use tree_sitter::{Node, Parser as TreeSitterParser};

struct Linter {
    parser: TreeSitterParser,
}

impl Linter {
    fn new() -> Result<Self, LinterError> {
        let mut parser = TreeSitterParser::new();
        let language = tree_sitter_r::language();
        parser
            .set_language(language)
            .map_err(|e| LinterError::Parse(e.to_string()))?;
        Ok(Self { parser })
    }

    fn lint(&mut self, code: &str) -> Result<Vec<SyntaxError>, LinterError> {
        let tree = self
            .parser
            .parse(code, None)
            .ok_or_else(|| LinterError::Parse("Failed to parse code".to_string()))?;
        let mut errors = Vec::new();
        let mut walker = tree.walk();
        self.find_errors(tree.root_node(), code, &mut errors, &mut walker);
        Ok(errors)
    }

    fn find_errors(&self, node: Node, code: &str, errors: &mut Vec<SyntaxError>, cursor: &mut tree_sitter::TreeCursor) {
        self.check_for_syntax_errors(&node, code, errors);
        self.check_for_arrow_assignment(&node, code, errors);

        if cursor.goto_first_child() {
            loop {
                self.find_errors(cursor.node(), code, errors, cursor);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    fn check_for_arrow_assignment(&self, node: &Node, _code: &str, errors: &mut Vec<SyntaxError>) {
        if node.kind() == "<-" {
            let start = node.start_position();
            errors.push(SyntaxError {
                line: start.row + 1,
                column: start.column + 1,
                message: "Use '=' for assignment instead of '<-'".to_string(),
            });
        }
    }

    fn check_for_syntax_errors(&self, node: &Node, code: &str, errors: &mut Vec<SyntaxError>) {
        if node.is_error() {
            let error_text = node.utf8_text(code.as_bytes()).unwrap_or("");
            let start_position = node.start_position();
            errors.push(SyntaxError {
                line: start_position.row + 1,
                column: start_position.column + 1,
                message: format!("Syntax error near '{}'", error_text),
            });
        } else if node.is_missing() {
            let start_position = node.start_position();
            errors.push(SyntaxError {
                line: start_position.row + 1,
                column: start_position.column + 1,
                message: format!("Missing {}", node.kind()),
            });
        }
    }
}

/// Lints the given R code and returns a list of syntax errors.
pub fn lint(code: &str) -> Result<Vec<SyntaxError>, LinterError> {
    let mut linter = Linter::new()?;
    linter.lint(code)
}