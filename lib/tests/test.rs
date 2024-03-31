use model_checker::KripkeBuilder;

#[test]
fn test() {
    let mut kripke_builder = KripkeBuilder::new();
    kripke_builder.add_state(vec!["a".to_string()], 0, true);
    kripke_builder.add_state(vec!["b".to_string()], 1, false);
    kripke_builder.add_transition(0, 1);
    kripke_builder.add_transition(1, 0);
    assert!(
        model_checker::ltl_model_check(kripke_builder.clone(), "G(a|b)")
            .unwrap()
            .is_none()
    );
    assert!(
        model_checker::ltl_model_check(kripke_builder.clone(), "G(a&b)")
            .unwrap()
            .is_some()
    );
}
