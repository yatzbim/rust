/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    create_buffer(0)
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut buffer = vec![0; count];
    buffer.truncate(count);
    buffer
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut buffer = create_buffer(5);
    fibonacci_helper(&mut buffer, 0);
    buffer
}

fn fibonacci_helper(buffer: &mut Vec<u8>, index: usize) {
    if index == 0 {
        buffer[0] = 1;
        buffer[1] = 1;
        fibonacci_helper(buffer, 2);
    } else if index >= buffer.len() {
        return;
    } else {
        let a = buffer.get(index - 2).unwrap();
        let b = buffer.get(index - 1).unwrap();
        buffer[index] = a + b;
        fibonacci_helper(buffer, index + 1);
    }
}
