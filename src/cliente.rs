use std::collections::HashSet;

#[derive(Debug)]
pub struct Cliente {
    pub id: u32,
    pub nome: String,
    pub compras: HashSet<u32>,
}

impl Cliente {
    pub fn novo(id: u32, nome: &str) -> Self {
        Cliente {
            id,
            nome: nome.to_string(),
            compras: HashSet::new(),
        }
    }

    pub fn adicionar_compra(&mut self, produto_id: u32) {
        self.compras.insert(produto_id);
    }

    pub fn ja_comprou(&self, produto_id: u32) -> bool {
        self.compras.contains(&produto_id)
    }
}