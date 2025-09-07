use crate::{LinterError, SyntaxError};
use tree_sitter::{Node, Parser as TreeSitterParser};
use tracing::{debug, error};

struct Linter {
    parser: TreeSitterParser,
}

impl Linter {
    fn new() -> Result<Self, LinterError> {
        let mut parser = TreeSitterParser::new();
        let language = tree_sitter_python::language();
        parser
            .set_language(language)
            .map_err(|e| LinterError::Parse(e.to_string()))?;
        Ok(Self { parser })
    }

    fn lint(&mut self, code: &str) -> Result<Vec<SyntaxError>, LinterError> {
        debug!("Attempting to parse code...");
        let tree = self
            .parser
            .parse(code, None)
            .ok_or_else(|| {
                error!("Failed to parse code into a syntax tree.");
                LinterError::Parse("Failed to parse code".to_string())
            })?;
        debug!("Code parsed successfully. Traversing syntax tree...");
        let mut errors = Vec::new();
        let mut walker = tree.walk();
        self.find_errors(tree.root_node(), code, &mut errors, &mut walker);
        debug!("Finished traversing syntax tree. Found {} errors.", errors.len());
        Ok(errors)
    }

    fn find_errors(&self, node: Node, code: &str, errors: &mut Vec<SyntaxError>, cursor: &mut tree_sitter::TreeCursor) {
        debug!("Visiting node: kind={}, text='{}', is_error={}, is_missing={}", 
               node.kind(), node.utf8_text(code.as_bytes()).unwrap_or(""), node.is_error(), node.is_missing());

        self.check_for_syntax_errors(&node, code, errors);
        self.check_for_print_statements(&node, code, errors);

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

    fn check_for_print_statements(&self, node: &Node, code: &str, errors: &mut Vec<SyntaxError>) {
        if node.kind() == "call" {
            if let Some(function_node) = node.child_by_field_name("function") {
                let function_name = function_node.utf8_text(code.as_bytes()).unwrap();
                if function_name == "print" {
                    let start = node.start_position();
                    debug!("Found discouraged print statement at line {}:{}", start.row + 1, start.column + 1);
                    errors.push(SyntaxError {
                        line: start.row + 1,
                        column: start.column + 1,
                        message: "Use of print statements is discouraged".to_string(),
                    });
                }
            }
        }
    }

    fn check_for_syntax_errors(&self, node: &Node, code: &str, errors: &mut Vec<SyntaxError>) {
        if node.is_error() {
            let error_text = node.utf8_text(code.as_bytes()).unwrap_or("");
            let start_position = node.start_position();
            error!("Tree-sitter reported an error node: kind={}, text='{}' at line {}:{}", 
                   node.kind(), error_text, start_position.row + 1, start_position.column + 1);
            errors.push(SyntaxError {
                line: start_position.row + 1,
                column: start_position.column + 1,
                message: format!("Syntax error near '{}'", error_text),
            });
        } else if node.is_missing() {
            let start_position = node.start_position();
            error!("Tree-sitter reported a missing node: kind={} at line {}:{}", 
                   node.kind(), start_position.row + 1, start_position.column + 1);
            errors.push(SyntaxError {
                line: start_position.row + 1,
                column: start_position.column + 1,
                message: format!("Missing {}", node.kind()),
            });
        }
    }
}

/// Lints the given Python code and returns a list of syntax errors.
pub fn lint(code: &str) -> Result<Vec<SyntaxError>, LinterError> {
    let mut linter = Linter::new()?;
    linter.lint(code)
}