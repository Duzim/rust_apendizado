use restaurant::front_of_house::hosting::add_to_waitlist;
use restaurant::teste::modulo_1::funcao;

// o use é práticamente um namespace, enquanto oque impota de verdade é o mod

// quando se utiliza o lib.rs e main.rs basicamente se cria duas arvores de modulos
// 

fn main() {
    let txt = String::from("TESTE 123");

    funcao(&txt);
    add_to_waitlist();
}
