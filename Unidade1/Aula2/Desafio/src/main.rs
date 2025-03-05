use std::ptr;

/// Estrutura que representa um nó na fila
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/// Estrutura que representa a fila
pub struct Queue<T> {
    front: Option<Box<Node<T>>>,    // Início da fila
    back: *mut Option<Box<Node<T>>>, // Ponteiro cru para o final da fila
    length: usize,                   // Contagem de elementos na fila
}

impl<T> Queue<T> {
    /// Cria e retorna uma nova fila vazia
    pub fn new() -> Self {
        Queue {
            front: None,
            back: std::ptr::null_mut(),
            length: 0,
        }
    }

    /// Insere um elemento no final da fila
    pub fn enqueue(&mut self, elem: T) {
        let new_node = Box::new(Node {
            value: elem,
            next: None,
        });

        // Se a fila estiver vazia
        if self.length == 0 {
            self.front = Some(new_node);
            self.back = &mut self.front.as_mut().unwrap().next as *mut _;
        } else {
            unsafe {
                // Usando unsafe para manipular o ponteiro de 'back'
                (*self.back).replace(new_node);
                self.back = &mut (*self.back).as_mut().unwrap().next as *mut _;
            }
        }
        self.length += 1;
    }

    /// Remove e retorna o elemento da frente da fila
    pub fn dequeue(&mut self) -> Option<T> {
        self.front.take().map(|node| {
            self.front = node.next;
            self.length -= 1;
            node.value
        })
    }

    /// Retorna uma referência ao elemento da frente da fila
    pub fn peek(&self) -> Option<&T> {
        self.front.as_ref().map(|node| &node.value)
    }

    /// Retorna o número de elementos na fila
    pub fn len(&self) -> usize {
        self.length
    }
}

/// Implementação do trait `Drop` para liberar a memória corretamente
impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.dequeue() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None); // Fila vazia
    }

    #[test]
    fn test_peek() {
        let mut queue: Queue<i32> = Queue::new();

        queue.enqueue(10);
        queue.enqueue(20);

        assert_eq!(queue.peek(), Some(&10));

        queue.dequeue();
        assert_eq!(queue.peek(), Some(&20));

        queue.dequeue();
        assert_eq!(queue.peek(), None); // Fila vazia
    }

    #[test]
    fn test_len() {
        let mut queue: Queue<i32> = Queue::new();

        assert_eq!(queue.len(), 0);

        queue.enqueue(1);
        assert_eq!(queue.len(), 1);

        queue.enqueue(2);
        assert_eq!(queue.len(), 2);

        queue.dequeue();
        assert_eq!(queue.len(), 1);

        queue.dequeue();
        assert_eq!(queue.len(), 0);
    }
}
