// Strange that rust doesn't seem have these...
pub trait Arity {
    const ARITY : usize;
    fn arity(&self) -> usize;
}

impl Arity for () {
    const ARITY: usize = 0;
    fn arity(&self) -> usize { 0 }
}

macro_rules! tuple_arity_impl { 
    ( $( ($($T:ident ),+ ) => $num:tt)+ ) => {
        $(
            impl<$($T, )+> Arity for ($($T, )+) {
                const ARITY: usize = $num;
                fn arity(&self) -> usize { Self::ARITY }        
            }
        )+
    }
}

tuple_arity_impl! {
    (A) => 1
    (A, B) => 2
    (A, B, C) => 3
    (A, B, C, D) => 4
    (A, B, C, D, E) => 5
    (A, B, C, D, E, F) => 6
    (A, B, C, D, E, F, G) => 7
    (A, B, C, D, E, F, G, H) => 8
    (A, B, C, D, E, F, G, H, I) => 9
    (A, B, C, D, E, F, G, H, I, J) => 10
    (A, B, C, D, E, F, G, H, I, J, K) => 11
}