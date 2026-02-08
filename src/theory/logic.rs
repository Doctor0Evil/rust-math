#[derive(Debug, Clone)]
pub enum Prop {
    Var(String),
    Not(Box<Prop>),
    And(Box<Prop>, Box<Prop>),
    Or(Box<Prop>, Box<Prop>),
    Implies(Box<Prop>, Box<Prop>),
}
