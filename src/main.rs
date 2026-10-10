use megastore::cliente::Cliente;
use megastore::grafo::Grafo;
use megastore::produto::Produto;

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=================================");
    println!("       MEGASTORE - CONECTASTORE");
    println!("=================================");

    // ==============================
    // CADASTRO DOS PRODUTOS
    // ==============================

    let mut produtos: HashMap<u32, Produto> = HashMap::new();

    produtos.insert(
        1,
        Produto::novo(
            1,
            "Notebook Acer",
            "Eletrônicos",
            3500.00,
        ),
    );

    produtos.insert(
        2,
        Produto::novo(
            2,
            "Mouse Gamer",
            "Eletrônicos",
            150.00,
        ),
    );

    produtos.insert(
        3,
        Produto::novo(
            3,
            "Teclado Mecânico",
            "Eletrônicos",
            300.00,
        ),
    );

    produtos.insert(
        4,
        Produto::novo(
            4,
            "Headset Gamer",
            "Eletrônicos",
            250.00,
        ),
    );

    produtos.insert(
        5,
        Produto::novo(
            5,
            "Monitor 240Hz",
            "Monitores",
            1800.00,
        ),
    );

    println!("\nProdutos cadastrados: {}", produtos.len());

    // ==============================
    // CADASTRO DO CLIENTE
    // ==============================

    let mut cliente = Cliente::novo(1, "Denis");

    cliente.adicionar_compra(1);
    cliente.adicionar_compra(2);

    println!("\nCliente: {}", cliente.nome);

    println!("Produtos comprados:");

    for id in &cliente.compras {
        if let Some(produto) = produtos.get(id) {
            println!(
                "- {} - R$ {:.2}",
                produto.nome,
                produto.preco
            );
        }
    }

    // ==============================
    // CRIAÇÃO DO GRAFO
    // ==============================

    let mut grafo = Grafo::novo();

    grafo.adicionar_aresta(1, 2, 0.9);
    grafo.adicionar_aresta(1, 3, 0.8);
    grafo.adicionar_aresta(1, 4, 0.7);
    grafo.adicionar_aresta(1, 5, 0.6);

    grafo.adicionar_aresta(2, 3, 0.8);
    grafo.adicionar_aresta(2, 4, 0.9);
    grafo.adicionar_aresta(3, 4, 0.8);
    grafo.adicionar_aresta(4, 5, 0.5);

    grafo.mostrar();

    // ==============================
    // BFS
    // ==============================

    let percurso = grafo.bfs(1);

    println!("\n=== PERCURSO BFS ===");
    println!("{:?}", percurso);

    // ==============================
    // RECOMENDAÇÕES
    // ==============================

    let inicio = Instant::now();

    let recomendacoes = grafo.recomendar(&cliente.compras);

    let tempo = inicio.elapsed();

    println!(
        "\n=== RECOMENDAÇÕES PARA {} ===",
        cliente.nome
    );

    if recomendacoes.is_empty() {
        println!("Nenhuma recomendação encontrada.");
    } else {
        for (posicao, id) in recomendacoes.iter().enumerate() {
            if let Some(produto) = produtos.get(id) {
                println!(
                    "{}. {} - R$ {:.2}",
                    posicao + 1,
                    produto.nome,
                    produto.preco
                );
            }
        }
    }

    println!(
        "\nTempo para gerar recomendações: {:?}",
        tempo
    );

    // ==============================
    // VERIFICAÇÃO
    // ==============================

    println!("\n=== VERIFICAÇÃO ===");

    for id in &cliente.compras {
        if cliente.ja_comprou(*id) {
            println!("Produto {} já comprado pelo cliente.", id);
        }
    }

       println!("\nSistema executado com sucesso!");

    teste_desempenho();
}

fn teste_desempenho() {
    println!("\n=================================");
    println!("       TESTE DE DESEMPENHO");
    println!("=================================");

    let tamanhos = [100, 1_000, 10_000];

    for tamanho in tamanhos {
        let mut grafo = Grafo::novo();

        for i in 1..=tamanho {
            grafo.adicionar_vertice(i);

            if i < tamanho {
                grafo.adicionar_aresta(i, i + 1, 0.8);
            }

            if i + 2 <= tamanho {
                grafo.adicionar_aresta(i, i + 2, 0.6);
            }
        }

        let mut compras = std::collections::HashSet::new();

        for i in (1..=tamanho).step_by(10) {
            compras.insert(i);
        }

        let inicio = std::time::Instant::now();

        let recomendacoes = grafo.recomendar(&compras);

        let tempo = inicio.elapsed();

        println!(
            "\nProdutos: {:>5} | Comprados: {:>5} | Recomendações: {:>5} | Tempo: {:?}",
            tamanho,
            compras.len(),
            recomendacoes.len(),
            tempo
        );
    }
}