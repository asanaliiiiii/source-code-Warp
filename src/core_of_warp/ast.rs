#[derive(Debug, Clone, PartialEq)]
pub enum WarpClass {
    Html(String),
    Style(String),
    Js(String),
    Variables(Vec<(String, String)>),
    Terminal(String),
    Tkinter(String),
    Other(String, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    StringLiteral(String),
    VariableRef(String),
    SysInfo,
    CmdWidget { x: i32, y: i32, size: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Import(String),
    ClassDef(String, WarpClass),
    Print(Expr),
    Transform(Expr, Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}