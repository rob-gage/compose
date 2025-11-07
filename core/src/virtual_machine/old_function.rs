// // Copyright Rob Gage 2025
// 
// use super::Term;
// 
// /// A function that takes a stack as its input and produces a stack as its output
// #[derive(Clone)]
// pub enum Function<'a> {
//     Contiguous { 
//         point
//     },
//     Composed (Vec<Term>)
// }
// 
// impl <'a> Function<'a> {
// 
//     /// Returns the `Term`s making up the body of this `Function`
//     pub fn body(&self) -> &[Term] {
//         match self {
//             Function::Contiguous (body) => body,
//             Function::Composed (body) => body
//         }
//     }
// 
//     /// Extends this `Function`'s body with additional `Term`s
//     pub fn extended(self, terms: &[Term]) -> Self {
//         let mut body: Vec<Term> = match self {
//             Function::Contiguous (body) => body.to_vec(),
//             Function::Composed (body) => body,
//         };
//         body.extend_from_slice(terms);
//         Function::Composed (body)
//     }
// 
// }