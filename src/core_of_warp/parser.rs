use crate::core_of_warp::lexer::{Lexer, Token};
use crate::core_of_warp::ast::{Program, Stmt, WarpClass, Expr};

pub struct Parser {
    lexer: Lexer,
    curr_tok: Token,
    peek_tok: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let curr = lexer.next_token();
        let peek = lexer.next_token();
        Self { lexer, curr_tok: curr, peek_tok: peek }
    }

    fn next_token(&mut self) {
        self.curr_tok = self.peek_tok.clone();
        self.peek_tok = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: Vec::new() };
        while self.curr_tok != Token::EOF {
            if let Some(stmt) = self.parse_statement() { program.statements.push(stmt); }
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Stmt> {
        match &self.curr_tok {
            Token::Import => {
                self.next_token();
                match &self.curr_tok {
                    Token::Identifier(name) | Token::StringLiteral(name) => Some(Stmt::Import(name.clone())),
                    _ => None,
                }
            }
            Token::Class => {
                self.next_token();
                if let Token::Identifier(class_name) = self.curr_tok.clone() {
                    self.next_token();
                    if self.curr_tok == Token::Semicolon {
                        self.next_token();
                        if let Token::StringLiteral(content) = self.curr_tok.clone() {
                            let class_obj = match class_name.as_str() {
                                "html" => WarpClass::Html(content),
                                "style" => WarpClass::Style(content),
                                "js" | "ts" => WarpClass::Js(content),
                                "terminal" => WarpClass::Terminal(content),
                                "tkinter" => WarpClass::Tkinter(content),
                                "variables" => self.parse_variables_block(&content),
                                _ => WarpClass::Other(class_name.clone(), content),
                            };
                            return Some(Stmt::ClassDef(class_name, class_obj));
                        }
                    }
                }
                None
            }
            Token::Print => self.parse_print_or_transform(),
            _ => None,
        }
    }

    fn parse_variables_block(&self, content: &str) -> WarpClass {
        let mut vars = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") { continue; }
            if let Some((key, val)) = line.split_once('=') {
                vars.push((key.trim().to_string(), val.trim().trim_matches('"').to_string()));
            }
        }
        WarpClass::Variables(vars)
    }

    fn parse_print_or_transform(&mut self) -> Option<Stmt> {
        self.next_token(); if self.curr_tok != Token::LeftParen { return None; }
        self.next_token();
        let left_expr = self.parse_expression();
        self.next_token(); if self.curr_tok == Token::RightParen { self.next_token(); }

        if self.curr_tok == Token::Trf {
            self.next_token();
            if self.curr_tok == Token::Print {
                self.next_token(); if self.curr_tok == Token::LeftParen {
                    self.next_token();
                    let right_expr = self.parse_expression();
                    self.next_token();
                    return Some(Stmt::Transform(left_expr, right_expr));
                }
            }
        }
        Some(Stmt::Print(left_expr))
    }

    fn parse_expression(&mut self) -> Expr {
        match &self.curr_tok {
            Token::StringLiteral(s) => Expr::StringLiteral(s.clone()),
            Token::Identifier(id) if id == "show.sysinfo" => Expr::SysInfo,
            Token::Identifier(id) if id == "cmd" => {
                let (mut x, mut y, mut size) = (0, 0, 400);
                while self.peek_tok != Token::Semicolon && self.peek_tok != Token::EOF && self.peek_tok != Token::RightParen {
                    self.next_token();
                    if let Token::Identifier(p) = &self.curr_tok {
                        let p_name = p.clone();
                        if self.peek_tok == Token::Equal {
                            self.next_token(); self.next_token();
                            if let Token::Identifier(v) = &self.curr_tok {
                                let val = v.parse::<i32>().unwrap_or(0);
                                if p_name == "x" { x = val; }
                                if p_name == "y" { y = val; }
                                if p_name == "size" { size = val; }
                            }
                        }
                    }
                }
                Expr::CmdWidget { x, y, size }
            }
            Token::Identifier(id) => Expr::VariableRef(id.clone()),
            _ => Expr::StringLiteral(String::new()),
        }
    }
}