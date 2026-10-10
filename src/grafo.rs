use std::collections::{HashMap, HashSet, VecDeque};

pub struct Grafo {
    adjacencias: HashMap<u32, Vec<(u32, f64)>>,
}

impl Grafo {
    pub fn novo() -> Self {
        Grafo {
            adjacencias: HashMap::new(),
        }
    }

    pub fn adicionar_vertice(&mut self, id: u32) {
        self.adjacencias.entry(id).or_insert_with(Vec::new);
    }

    pub fn adicionar_aresta(&mut self, origem: u32, destino: u32, peso: f64) {
        self.adicionar_vertice(origem);
        self.adicionar_vertice(destino);

        self.adjacencias
            .get_mut(&origem)
            .unwrap()
            .push((destino, peso));
    }

    pub fn mostrar(&self) {
        println!("\n=== GRAFO DA MEGASTORE ===");

        for (vertice, conexoes) in &self.adjacencias {
            print!("{} -> ", vertice);

            for &(destino, peso) in conexoes {
                print!("{} (peso: {:.1}) ", destino, peso);
            }

            println!();
        }
    }

    pub fn bfs(&self, inicio: u32) -> Vec<u32> {
        let mut visitados = HashSet::new();
        let mut fila = VecDeque::new();
        let mut resultado = Vec::new();

        fila.push_back(inicio);
        visitados.insert(inicio);

        while let Some(atual) = fila.pop_front() {
            resultado.push(atual);

            if let Some(vizinhos) = self.adjacencias.get(&atual) {
                for &(vizinho, _) in vizinhos {
                    if !visitados.contains(&vizinho) {
                        visitados.insert(vizinho);
                        fila.push_back(vizinho);
                    }
                }
            }
        }

        resultado
    }

    pub fn recomendar(
        &self,
        produtos_comprados: &HashSet<u32>,
    ) -> Vec<u32> {
        let mut recomendacoes: HashMap<u32, f64> = HashMap::new();

        for &produto_comprado in produtos_comprados {
            if let Some(vizinhos) = self.adjacencias.get(&produto_comprado) {
                for &(produto, peso) in vizinhos {
                    if !produtos_comprados.contains(&produto) {
                        let entrada = recomendacoes.entry(produto).or_insert(0.0);

                        if peso > *entrada {
                            *entrada = peso;
                        }
                    }
                }
            }
        }

        let mut lista: Vec<(u32, f64)> =
            recomendacoes.into_iter().collect();

        lista.sort_by(|a, b| {
            b.1.partial_cmp(&a.1).unwrap()
        });

        lista
            .into_iter()
            .map(|(produto, _peso)| produto)
            .collect()
    }
}


// ======================================================
// TESTES UNITÁRIOS
// ======================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_bfs() {
        let mut grafo = Grafo::novo();

        grafo.adicionar_aresta(1, 2, 0.9);
        grafo.adicionar_aresta(1, 3, 0.8);
        grafo.adicionar_aresta(2, 4, 0.7);

        let resultado = grafo.bfs(1);

        assert_eq!(resultado, vec![1, 2, 3, 4]);
    }

    #[test]
    fn teste_recomendacao_nao_recomenda_produto_comprado() {
        let mut grafo = Grafo::novo();

        grafo.adicionar_aresta(1, 2, 0.9);
        grafo.adicionar_aresta(1, 3, 0.8);

        let mut comprados = HashSet::new();

        comprados.insert(1);
        comprados.insert(2);

        let recomendacoes = grafo.recomendar(&comprados);

        assert!(!recomendacoes.contains(&1));
        assert!(!recomendacoes.contains(&2));
        assert!(recomendacoes.contains(&3));
    }

    #[test]
    fn teste_recomendacao_sem_duplicados() {
        let mut grafo = Grafo::novo();

        grafo.adicionar_aresta(1, 3, 0.8);
        grafo.adicionar_aresta(2, 3, 0.9);

        let mut comprados = HashSet::new();

        comprados.insert(1);
        comprados.insert(2);

        let recomendacoes = grafo.recomendar(&comprados);

        assert_eq!(recomendacoes.len(), 1);
        assert_eq!(recomendacoes[0], 3);
    }
}