macro_rules! create_expr_struct {
    ($struct_name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            pub pos: Position,
            pub children: Vec<Box<DalaExpression>>,
        }

        impl $struct_name {
            pub fn new(pos: Position, children: Vec<Box<DalaExpression>>) -> Self {
                Self { pos, children }
            }
        }
    };
}

pub(crate) use create_expr_struct;
