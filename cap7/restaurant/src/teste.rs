pub mod modulo_1 {
    pub fn funcao(text: &str) {
        println!("Função que faz algo [{}]", text);
    }
}

pub enum EnumTeste<T, U> {
    Teste(T),
    NaoTeste(U),
    Aaaaaa,
}

pub struct Customer {
    pub nome: String,
    idade: i8,
}
