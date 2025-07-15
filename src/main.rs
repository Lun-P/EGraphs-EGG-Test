use egg::{
    AstSize, EGraph, Extractor, Pattern, RecExpr, Rewrite, Runner, Searcher, SymbolLang, rewrite,
};

mod rustbegin;

fn main() {
    rustbegin::printing();
    rustbegin::types();

    //egg_tests();
}

// EGG Library Tests
#[allow(dead_code)]
fn egg_tests() {
    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rewrite!("add-cmt"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        rewrite!("mul-cmt"; "(* ?x ?y)" => "(* ?y ?x)"),
        rewrite!("add-0"; "(+ ?x 0)" => "?x"),
        rewrite!("mul-0"; "(* ?x 0)" => "0"),
        rewrite!("mul-1"; "(* ?x 1)" => "?x"),
    ];

    let start = "(+ 0 (* 1 a))".parse().unwrap();
    let runner = Runner::default().with_expr(&start).run(rules);
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
    println!("Best Expression: {}\nWith AstSize cost {}", best_expr, best_cost);

    let myexpr: RecExpr<SymbolLang> = match "(+ a b)".parse() {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    let mut egraph: EGraph<SymbolLang, ()> = Default::default();
    let a = egraph.add(SymbolLang::leaf("a"));
    let b = egraph.add(SymbolLang::leaf("b"));
    let foo = egraph.add(SymbolLang::new("+", vec![a, b]));
    let foo2 = egraph.add_expr(&myexpr);
    assert_eq!(&foo, &foo2);

    egraph.rebuild();

    let pattern: Pattern<SymbolLang> = "(+ ?x ?y)".parse().unwrap();
    let matches = pattern.search(&egraph);
    assert!(!matches.is_empty());
}
