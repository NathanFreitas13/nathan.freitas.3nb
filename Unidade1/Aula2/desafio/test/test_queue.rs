use queue::queue::Queue;  // Importa o módulo Queue do projeto



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_peek() {
        let mut queue = Queue::new();
        queue.enqueue(42);
        assert_eq!(queue.peek(), Some(&42));
    }

    #[test]
    fn test_len() {
        let mut queue = Queue::new();
        assert_eq!(queue.len(), 0);
        queue.enqueue(10);
        queue.enqueue(20);
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_is_empty() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());
        queue.enqueue(5);
        assert!(!queue.is_empty());
    }
}