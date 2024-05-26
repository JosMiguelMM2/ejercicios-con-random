use colored::*; // Librería para imprimir texto en color en la consola.
use rand::Rng; // Librería para generar números aleatorios.
use std::collections::VecDeque; // Cola de doble extremo para manejar asignaciones.

// Definición de la estructura de datos que contiene el número de estaciones y habitantes por barrio.
#[derive(Debug)]
struct Data {
    estaciones: i32,
    habitantes: Vec<i32>,
}

// Función para generar datos aleatorios de habitantes y estaciones.
fn generate_data(numero_barrios: i32) -> Data {
    let mut random = rand::thread_rng(); // Generador de números aleatorios.
    Data {
        estaciones: random.gen_range(2..6), // Genera un número aleatorio de estaciones entre 2 y 5.
        habitantes: (0..numero_barrios)
            .map(|_| random.gen_range(1..501)) // Genera un número aleatorio de habitantes entre 1 y 500 para cada barrio.
            .collect(),
    }
}

// Función para particionar los habitantes entre las estaciones de manera equitativa.
fn partition(mut habitantes: Vec<i32>, estaciones: i32) -> (bool, i32, Vec<Vec<i32>>) {
    habitantes.sort_unstable_by(|a, b| b.cmp(a)); // Ordena los habitantes de mayor a menor.
    let mut queues: Vec<VecDeque<i32>> = vec![VecDeque::new(); estaciones as usize]; // Vector de colas para almacenar las asignaciones.
    let mut sums: Vec<i32> = vec![0; estaciones as usize]; // Vector de sumas para rastrear el total de habitantes por estación.

    for habitante in habitantes {
        // Encuentra la estación con la menor suma actual y asigna el habitante a esa estación.
        let min_index = sums
            .iter()
            .enumerate()
            .min_by_key(|&(_, sum)| sum)
            .unwrap()
            .0;
        queues[min_index].push_back(habitante);
        sums[min_index] += habitante;
    }

    // Determina si todas las sumas son iguales para verificar una distribución equitativa.
    let max_sum = *sums.iter().max().unwrap();
    let all_equal = sums.iter().all(|&sum| sum == max_sum);

    (
        all_equal,
        max_sum,
        queues.into_iter().map(|q| q.into()).collect(), // Convierte las colas en vectores.
    )
}

fn main() {
    let mut rg = rand::thread_rng(); // Generador de números aleatorios.

    println!(" ");
    println!("{}", 
            format!("Asignación de estaciones de un sistema de transporte masivo según los habitantes de los barrios  ").color("cyan"));

    // Realiza 50 iteraciones del experimento.
    for _ in 0..50 {
        let num_barrios: i32 = rg.gen_range(2..100); // Genera un número aleatorio de barrios entre 2 y 99.
        let generate: Data = generate_data(num_barrios); // Genera los datos aleatorios.
        
        println!("Número de barrios {} ", num_barrios);
        println!(
            "{}",
            format!("Número estaciones: {}", generate.estaciones).color("yellow")
        );
        println!(
            "{}",
            format!("Número de personas por barrio: {:?}", generate.habitantes).color("white")
        );

        // Particiona los habitantes entre las estaciones.
        let (flag, sumatoria, particiones) = partition(generate.habitantes, generate.estaciones);

        if !flag {
            // Si no es posible una distribución equitativa, muestra un mensaje en rojo.
            println!(
                "{}",
                format!("No se pueden distribuir con los valores dados").color("red")
            );
            let mut num_estacion = 1;
            for estacion in &particiones {
                println!(
                    "{}",
                    format!(
                        "Estación número: {} asignación: {:?}",
                        num_estacion, estacion
                    )
                    .color("red")
                );
                num_estacion = num_estacion + 1;
            }
        } else {
            // Si es posible una distribución equitativa, muestra un mensaje en verde.
            println!("{}", format!("Si se puede distribuir de manera equitativa, el valor de cada asignación es: {}", sumatoria).color("green"));

            let mut num_estacion = 1;
            for estacion in &particiones {
                println!(
                    "{}",
                    format!(
                        "Estación número: {} asignación: {:?}",
                        num_estacion, estacion
                    )
                    .color("green")
                );
                num_estacion = num_estacion + 1;
            }
        }
        println!(" ");
        println!(" ");
    }
}
