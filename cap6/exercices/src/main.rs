enum EventoJanela {
    CliqueNoMouse(i32, i32), // x, y
    Teclado(char),
    Sair,
}

fn processar_evento_log(evento: EventoJanela) {
    if let EventoJanela::Teclado(c) = evento {
        println!("Tecla pressionada: {}", c);
    }
}

fn main() {
    let evento1 = EventoJanela::CliqueNoMouse(10, 20);
    let evento2 = EventoJanela::Teclado('R');
    let evento3 = EventoJanela::Sair;

    processar_evento_log(evento1);
    processar_evento_log(evento2); // Apenas este deve imprimir algo
    processar_evento_log(evento3);
}
