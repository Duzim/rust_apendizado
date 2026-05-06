pub mod teste;

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Adiciona a lista");
        }

        pub fn seat_at_table() {
            println!("seat_at_table...");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Pega pedido");
        }

        pub fn serve_order() {
            println!("Salvando pedido..");
        }

        pub fn take_payment() {
            println!("Pegando pedido...")
        }
    }
}

pub fn funcao_no_topo() {
    println!("Chegamos ao topo!");
}

//Sim, isso é possivel (mesmo que não recomendado)
pub mod nivel_1 {
    pub mod nivel_2 {
        pub mod nivel_3 {
            pub mod nivel_4 {
                pub fn tentar_acessar_o_topo() {
                    // Subindo 4 níveis!
                    super::super::super::super::funcao_no_topo();
                }
            }
        }
    }
}
