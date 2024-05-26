use colored::*;
use rand::Rng;
use std::collections::HashMap;

const CANTIDAD_PACIENTES: usize = 3000;
const CLINICAS: usize = 2;

struct SignosVitales {
    temperatura_corporal: f32,
    presion_arterial: (u8, u8),
    frecucencia_cardiaca: u8,
    fecuencia_respiratori: u8,
}

impl SignosVitales {
    fn esnormal(&self) -> bool {
        let temperatura_normal =
            self.temperatura_corporal >= 36.5 && self.temperatura_corporal <= 37.5;
        let presion_nomla = self.presion_arterial.0 >= 90
            && self.presion_arterial.0 <= 120
            && self.presion_arterial.1 >= 60
            && self.presion_arterial.1 <= 80;
        let frecuencia_respi_normal =
            self.fecuencia_respiratori >= 12 && self.fecuencia_respiratori >= 20;
        let frecuencia_normal_cardiaca =
            self.frecucencia_cardiaca >= 60 && self.frecucencia_cardiaca <= 120;

        temperatura_normal && presion_nomla && frecuencia_normal_cardiaca && frecuencia_respi_normal
    }
}

fn generar_signos_random() -> SignosVitales {
    let mut random = rand::thread_rng();
    SignosVitales {
        temperatura_corporal: random.gen_range(30.0..39.0),
        presion_arterial: (random.gen_range(40..150), random.gen_range(40..100)),
        frecucencia_cardiaca: random.gen_range(40..120),
        fecuencia_respiratori: random.gen_range(8..30),
    }
}

fn imprimir(pacientes: &Vec<&SignosVitales>, color: &str) {
    for paciente in pacientes {
        println!("{}",
            format!(
            "Temperatura: {:.1}°C,      Presión arterial: {}/{} mmHg,     Frecuencia cardíaca: {} bpm,       Frecuencia respiratoria: {} rpm",
            paciente.temperatura_corporal, paciente.presion_arterial.0, paciente.presion_arterial.1, paciente.frecucencia_cardiaca, paciente.fecuencia_respiratori
            ).color(color))
    }
    println!(" ");
}

fn main() {
    let mut pacientes_por_clinica: HashMap<usize, Vec<(SignosVitales, bool)>> = HashMap::new();

    for _ in 0..CANTIDAD_PACIENTES {
        let clinica = rand::thread_rng().gen_range(0..CLINICAS);
        let signos_vitales_g: SignosVitales = generar_signos_random();
        let is_normal = !signos_vitales_g.esnormal();

        pacientes_por_clinica
            .entry(clinica)
            .or_insert(Vec::new())
            .push((signos_vitales_g, is_normal));
    }

    for (clinica, pacientes) in &pacientes_por_clinica {
        println!("{}", format!("Clinica {}  ", clinica).color("blue"));

        let pacientes_anormales: Vec<_> = pacientes
            .iter()
            .filter(|(_, is_anormal)| *is_anormal)
            .map(|(signos_vitales_g, _)| signos_vitales_g)
            .collect();
        let pacientes_normales: Vec<_> = pacientes
            .iter()
            .filter(|(_, is_anormal)| !is_anormal)
            .map(|(signos_vitales_g, _)| signos_vitales_g)
            .collect();

        println!(
            "Cantidad de pacientes con signos vitales anormales: {}",
            pacientes_anormales.len()
        );

        println!("Pacientes con signos vitales anormales:");

        imprimir(&pacientes_anormales, "yellow");

        println!(" ");
        println!(
            "Cantidad de pacientes con signos vitales normales: {}",
            pacientes_normales.len()
        );
        println!("\nPacientes con signos vitales normales:");

        imprimir(&pacientes_normales, "green");
    }
}
