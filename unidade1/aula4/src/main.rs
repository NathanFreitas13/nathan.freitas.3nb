/*fn main () {
    let a = 50;
    let mut b = 100;
    let c = &mut b;
    *c *= 2;
    println!("O valor de c é: {}", c);
}
*/

fn main() {
    let mut num = 5;
    // Cria um ponteiro cru para num.
    let r1: *mut i32 = &mut num;

    // Bloco unsafe para desreferenciar o ponteiro.
    unsafe {
        *r1 += 1;
        println!("Valor modificado via ponteiro cru: {}", *r1);
    }
}
