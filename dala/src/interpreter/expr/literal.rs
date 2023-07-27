use crate::{DalaError, DalaValue, Position};

use super::eval_visitor::EvalVisitor;

macro_rules! create_lit_struct {
    ($struct_name:ident, $type:ty) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            pub pos: Position,
            pub value: $type,
        }

        impl $struct_name {
            pub fn new(pos: Position, value: $type) -> Self {
                Self { pos, value }
            }
        }
    };
}

create_lit_struct!(Str, String);
create_lit_struct!(Num, f64);
create_lit_struct!(Bool, bool);

impl EvalVisitor for Str {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Str(self.value.clone()))
    }
}

impl EvalVisitor for Num {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Num(self.value))
    }
}

impl EvalVisitor for Bool {
    fn eval(&self) -> Result<DalaValue, DalaError> {
        Ok(DalaValue::Boolean(self.value))
    }
}
