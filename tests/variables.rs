/*
 * Variables
 */

extern crate rustyard;

use std::collections::HashMap;

#[test]
fn can_use_variables() {
    let mut vars = HashMap::new();
    let mut yard = rustyard::ShuntingYard::new();

    for (k, v) in vec![(String::from("a"), 5.0), (String::from("b"), 2.0)] {
        vars.insert(k, v);
    }

    assert_eq!(5.0, yard.calculate_with_variables("max(a, b)", &vars).unwrap());
    assert_eq!("max ( 5 , 2 ) ", yard.to_string_ast());
    assert_eq!("5 2 max ", yard.to_string());
}
