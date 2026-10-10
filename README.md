# MegaStore - ConectaStore

Sistema de recomendação de produtos desenvolvido em Rust para a plataforma fictícia de e-commerce MegaStore.

O projeto utiliza estruturas de dados e algoritmos de grafos para analisar relações entre produtos e gerar recomendações personalizadas com base no histórico de compras de um cliente.

---

## 1. Objetivo

O objetivo do projeto é desenvolver um sistema capaz de recomendar produtos para clientes de uma plataforma de comércio eletrônico.

Para isso, os produtos são representados como vértices de um grafo e as relações entre eles são representadas por arestas ponderadas.

O peso de cada aresta representa o grau de relação ou similaridade entre dois produtos.

O sistema analisa os produtos já comprados pelo cliente e procura produtos relacionados que ainda não foram adquiridos.

---

## 2. Funcionalidades

O sistema possui as seguintes funcionalidades:

- Cadastro de produtos;
- Cadastro de clientes;
- Registro do histórico de compras;
- Armazenamento dos produtos utilizando `HashMap`;
- Armazenamento das compras utilizando `HashSet`;
- Construção de um grafo de produtos;
- Criação de relações ponderadas entre produtos;
- Representação do grafo através de lista de adjacência;
- Percurso utilizando o algoritmo BFS (Busca em Largura);
- Geração de recomendações personalizadas;
- Prevenção de recomendações de produtos já comprados;
- Prevenção de recomendações duplicadas;
- Ordenação das recomendações pelo peso da relação;
- Testes automatizados;
- Medição do tempo de execução;
- Testes de desempenho com diferentes volumes de produtos.

---

## 3. Tecnologias utilizadas

O projeto foi desenvolvido utilizando:

- Rust;
- Cargo;
- HashMap;
- HashSet;
- VecDeque;
- Grafos;
- Lista de adjacência;
- Algoritmo BFS;
- `std::time::Instant` para medição de desempenho.

---

## 4. Estrutura do projeto

```text
megastore/
├── Cargo.toml
├── README.md
├── src/
│   ├── cliente.rs
│   ├── grafo.rs
│   ├── lib.rs
│   ├── main.rs
│   └── produto.rs
│
└── tests/
    └── integracao.rs

    ## 5. Modelagem do Grafo

O sistema utiliza um grafo para representar as relações entre os produtos da MegaStore.

### Vértices

Cada produto cadastrado no sistema é representado por um vértice do grafo.

O identificador do produto (`u32`) é utilizado para identificar cada vértice.

### Arestas

As arestas representam relações entre produtos que podem ser considerados relacionados ou recomendados entre si.

Cada aresta possui um peso (`f64`) que representa o grau de relação entre os produtos.

Quanto maior o peso, maior é a relevância da relação entre os produtos.

Exemplo:

```text
1 -> 2 (peso: 0.9)
1 -> 3 (peso: 0.8)
1 -> 4 (peso: 0.7)