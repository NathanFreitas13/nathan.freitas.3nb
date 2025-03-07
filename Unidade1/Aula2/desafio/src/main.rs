mod queue; // Importa o módulo queue.rs

use queue::Queue; 

fn main() {
    // Criando uma nova fila
    let mut queue = Queue::new();

    // Adicionando elementos à fila
    queue.enqueue(10);
    queue.enqueue(20);

    // Verificando se a fila está vazia.
    println!("A fila está vazia: {}", queue.is_empty());  // Tradução da mensagem

    // Exibindo o tamanho da fila
    println!("Tamanho da fila: {}", queue.len());  // Tradução da mensagem

    // Verificando o primeiro elemento da fila sem removê-lo
    println!("Primeiro elemento (peek): {:?}", queue.peek());  // Tradução da mensagem

    // Removendo um elemento da fila
    queue.dequeue();

    // Exibindo o tamanho da fila após a remoção
    println!("Tamanho da fila após remoção: {}", queue.len());  // Tradução da mensagem
}