use crate::{LinterError, SyntaxError};
use tree_sitter::{Node, Parser as TreeSitterParser};

struct Linter {
    parser: TreeSitterParser,
}

impl Linter {
    fn new() -> Result<Self, LinterError> {
        let mut parser = TreeSitterParser::new();
        let language = tree_sitter_java::language();
        parser
            .set_language(language)
            .map_err(|e| LinterError::TreeSitterParseError(format!("Failed to set tree-sitter language for Java: {}", e)))?;
        Ok(Self { parser })
    }

    fn lint(&mut self, code: &str) -> Result<Vec<SyntaxError>, LinterError> {
        let tree = self
            .parser
            .parse(code, None)
            .ok_or_else(|| LinterError::TreeSitterParseError("Tree-sitter failed to parse the entire file. This may indicate highly unusual syntax or an internal tree-sitter issue.".to_string()))?;

        let mut errors = Vec::new();
        let mut walker = tree.walk();
        self.find_errors(tree.root_node(), code, &mut errors, &mut walker);
        Ok(errors)
    }

    fn find_errors(&self, node: Node, code: &str, errors: &mut Vec<SyntaxError>, cursor: &mut tree_sitter::TreeCursor) {
        self.check_for_syntax_errors(&node, code, errors);
        self.check_for_keyword_identifier(&node, code, errors);
        self.check_for_invalid_constructor(&node, code, errors);

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

    fn is_keyword(&self, s: &str) -> bool {
        matches!(s,
            "abstract" | "continue" | "for" | "new" | "switch" | "assert" | "default" | "goto" |
            "package" | "synchronized" | "boolean" | "do" | "if" | "private" | "this" | "break" |
            "double" | "implements" | "protected" | "throw" | "byte" | "else" | "import" | "public" |
            "throws" | "case" | "enum" | "instanceof" | "return" | "transient" | "catch" | "extends" |
            "int" | "short" | "try" | "char" | "final" | "interface" | "static" | "void" | "class" |
            "finally" | "long" | "strictfp" | "volatile" | "const" | "float" | "native" | "super" |
            "while" | "true" | "false" | "null"
        )
    }

    fn check_for_keyword_identifier(&self, node: &Node, code: &str, errors: &mut Vec<SyntaxError>) {
        if node.kind() == "identifier" {
            let identifier = node.utf8_text(code.as_bytes()).unwrap();
            if self.is_keyword(identifier) {
                let start = node.start_position();
                errors.push(SyntaxError {
                    line: start.row + 1,
                    column: start.column + 1,
                    message: format!("'{}' is a keyword and cannot be used as an identifier", identifier),
                });
            }
        }
    }

    fn check_for_invalid_constructor(&self, node: &Node, code: &str, errors: &mut Vec<SyntaxError>) {
        if node.kind() == "constructor_declaration" {
            let mut parent = node.parent();
            let mut is_in_enum = false;
            while let Some(p) = parent {
                if p.kind() == "enum_declaration" {
                    is_in_enum = true;
                    break;
                }
                if p.kind() == "class_declaration" {
                    break;
                }
                parent = p.parent();
            }

            if !is_in_enum {
                let mut constructor_name = "";
                for child in node.children(&mut node.walk()) {
                    if child.kind() == "identifier" {
                        constructor_name = child.utf8_text(code.as_bytes()).unwrap();
                        break;
                    }
                }

                let mut class_name = "";
                parent = node.parent(); // Reset parent
                while let Some(p) = parent {
                    if p.kind() == "class_declaration" {
                        for child in p.children(&mut p.walk()) {
                            if child.kind() == "identifier" {
                                class_name = child.utf8_text(code.as_bytes()).unwrap();
                                break;
                            }
                        }
                        break;
                    }
                    parent = p.parent();
                }

                if !class_name.is_empty() && constructor_name != class_name {
                    let start = node.start_position();
                    errors.push(SyntaxError {
                        line: start.row + 1,
                        column: start.column + 1,
                        message: format!("Invalid constructor name '{}'. Constructor name must match the class name '{}'", constructor_name, class_name),
                    });
                }
            }
        }
    }
}

pub fn lint(code: &str) -> Result<Vec<SyntaxError>, LinterError> {
    let mut linter = Linter::new()?;
    linter.lint(code)
}