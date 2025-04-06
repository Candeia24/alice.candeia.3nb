#[cfg(test)]
mod bst_tests {
    use crate::BST;
    // Importe sua implementação de BST aqui
    // use crate::BST;
    
    #[test]
    fn test_bst_new_and_empty() {
        // Teste 1: Criar uma nova árvore e verificar se está vazia
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        // Teste 2: Inserir elementos e verificar se estão na árvore
        let mut bst = BST::new();
        
        // Inserir alguns valores
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        // Verificar se os valores inseridos estão na árvore
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        
        // Verificar que um valor não inserido não está na árvore
        assert!(!bst.search(20));
        
        // A árvore não deve mais estar vazia
        assert!(!bst.is_empty());
    }
}


#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

// Estrutura de uma Árvore Binária de Busca (BST)
pub struct BST {
    pub root: Option<Box<Node>>,
}

impl BST {
    // Cria uma nova árvore de busca vazia
    pub fn new() -> Self {
        BST { root: None }
    }

    // Ve se a árvore está vazia
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    // Insere um novo valor na árvore
    pub fn insert(&mut self, value: i32) {
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(current: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        // Caso a árvore esteja vazia cria um novo nó
        match current {
            None => Some(Box::new(Node { value, left: None, right: None })),
            // Se a árvore não estiver vazia ele coloca o valor no nó adequado 
            Some(mut node) => {
                // Se o valor for menor que o do nó atual, insere na subárvore esquerda
                if value < node.value {
                    node.left = Self::insert_node(node.left.take(), value);
                } 
                // Se o valor for maior que o do nó atual, insere na subárvore direita
                else if value > node.value {
                    node.right = Self::insert_node(node.right.take(), value);
                }
                // Retorna o nó atualizado
                Some(node)
            }
        }
    }

    // Busca um valor na árvore
    pub fn search(&self, value: i32) -> bool {
        Self::search_node(&self.root, value)
    }

    fn search_node(current: &Option<Box<Node>>, value: i32) -> bool {
        // se a árvore estiver vazia, o valor não foi encontrado
        match current {
            None => false,
            // Caso a árvore não esteja vazia, procura o valor do nó 
            Some(node) => {
                // Se o valor do nó atual for igual ao valor buscado, retorna verdade(true)
                if node.value == value {
                    return true
                } 
                // Se o valor for menor que o do nó atual, busca na subárvore esquerda
                else if value < node.value {
                    Self::search_node(&node.left, value)
                } 
                // Se o valor for maior que o do nó atual, busca na subárvore direita
                else {
                    Self::search_node(&node.right, value)
                }
            }
        }
    }
}
