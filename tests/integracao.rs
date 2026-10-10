#[test]
fn teste_soma() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn teste_nao_recomenda_produto_comprado() {
    let produtos_comprados = vec![1, 2];

    let recomendacoes = vec![3, 4, 5];

    for produto in recomendacoes {
        assert!(!produtos_comprados.contains(&produto));
    }
}