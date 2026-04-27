fn main() {
    let p1 = PontoGeografico::novo(-23.5505, -46.6333); // SP
    let p2 = PontoGeografico::novo(-22.9068, -43.1729); // RJ

    let bounding_box = CaixaDelimitadora {
        canto_inferior_esquerdo: p1,
        canto_superior_direito: p2,
    };

    println!("Largura da região: {:.4} graus", bounding_box.largura());
    println!("Altura da região: {:.4} graus", bounding_box.altura());
}

struct PontoGeografico {
    latitude: f64,
    longitude: f64,
}
impl PontoGeografico {
    fn novo(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

struct CaixaDelimitadora {
    canto_inferior_esquerdo: PontoGeografico,
    canto_superior_direito: PontoGeografico,
}
impl CaixaDelimitadora {
    fn largura(&self) -> f64 {
        (self.canto_inferior_esquerdo.latitude - self.canto_superior_direito.longitude).abs()
    }

    fn altura(&self) -> f64 {
        (self.canto_superior_direito.latitude - self.canto_inferior_esquerdo.latitude).abs()
    }
}
