// Copyright Rob Gage 2025

use crate::Integer;

/// Data that can be stored on the `Stack`
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Data {

    /// A true or false value
    Boolean (bool),

    /// An unbounded signed integer
    Integer (Integer),

    /// An anonymous function
    Lambda (Vec<usize>),

    /// A list of `Data`
    List (Vec<Data>),

    /// A text string
    String (String),

}

// impl Data {
//
//     pub fn display(&self, namespace: &Namespace) -> String {
//         match self {
//
//             Data::Boolean (boolean) => format!("{}", if *boolean { "true" } else { "false" }),
//
//             Data::Integer (integer) => format!("{}", integer.to_string()),
//
//             Data::Lambda (terms) => format!("( {} )", terms.iter()
//                 .map(|term| namespace.display_term(term))
//                 .collect::<Vec<String>>()
//                 .join(" ")
//             ),
//
//             Data::List (items) => format!("[ {} ]", items.iter()
//                 .map(|item| item.display(namespace))
//                 .collect::<Vec<_>>()
//                 .join(" ")
//             ),
//
//             Data::String (string) => format!("\"{}\"", string),
//
//         }
//     }
//
// }