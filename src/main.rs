// Interval Partitioning: prova de conceito do algoritmo guloso com heap.
//
// Parte 1: resolve o LeetCode 2406 (Divide Intervals Into Minimum Number
//          of Groups) e imprime cada decisão gulosa.
// Parte 2: aplica o mesmo algoritmo a uma agenda de reuniões, mostrando a
//          atribuição de salas numa linha do tempo e conferindo o resultado
//          com uma varredura independente (sweep line).
//
// Uso:
//   cargo run                         # agenda padrão
//   cargo run -- 1-5 5-10 2-4 3-7     # intervalos personalizados (inicio-fim)

mod leetcode;
mod scheduler;

use scheduler::Interval;

/// Agenda padrão. Note o par 1-5 / 5-10: os extremos são inclusivos, então
/// eles conflitam (ambos ocupam o instante 5).
const DEFAULT_AGENDA: [Interval; 8] = [
    (1, 5),
    (5, 10),
    (2, 4),
    (3, 7),
    (6, 8),
    (8, 12),
    (9, 11),
    (11, 13),
];

/// Lê os intervalos da linha de comando no formato `inicio-fim`.
fn parse_args() -> Result<Vec<Interval>, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        return Ok(DEFAULT_AGENDA.to_vec());
    }
    args.iter()
        .map(|a| {
            let (s, e) = a
                .split_once('-')
                .ok_or(format!("formato inválido '{}': use inicio-fim, ex.: 1-5", a))?;
            let s: i32 = s.parse().map_err(|_| format!("início inválido em '{}'", a))?;
            let e: i32 = e.parse().map_err(|_| format!("fim inválido em '{}'", a))?;
            if s > e {
                return Err(format!("início maior que o fim em '{}'", a));
            }
            Ok((s, e))
        })
        .collect()
}

fn main() {
    let agenda = match parse_args() {
        Ok(a) => a,
        Err(msg) => {
            eprintln!("erro: {}", msg);
            std::process::exit(1);
        }
    };

    // ---------- Parte 1: LeetCode 2406 ----------
    println!("=== Parte 1: LeetCode 2406 (Divide Intervals Into Minimum Number of Groups) ===");
    // Os dois exemplos oficiais do enunciado.
    let examples = [
        vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]],
        vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]],
    ];
    for intervals in examples {
        println!("intervals = {:?}", intervals);
        let answer = leetcode::min_groups_verbose(intervals);
        println!("  mínimo de grupos: {}\n", answer);
    }

    // ---------- Parte 2: agenda de reuniões ----------
    println!("=== Parte 2: agenda de reuniões ===");
    println!("Intervalos (fim inclusivo):");
    for (i, (s, e)) in agenda.iter().enumerate() {
        println!("  {} = [{},{}]", scheduler::id_of(i), s, e);
    }

    // Decisões gulosas, em ordem de início.
    println!("\nPassos gulosos (ordenado pelo início):");
    let groups = scheduler::partition(&agenda, true);

    println!("\nLinha do tempo:");
    scheduler::print_gantt(&agenda, &groups);

    // ---------- Conferência ----------
    // 1) Mesmo número de grupos que a solução do LeetCode.
    let as_vecs: Vec<Vec<i32>> = agenda.iter().map(|&(s, e)| vec![s, e]).collect();
    let lc_answer = leetcode::min_groups(as_vecs) as usize;
    // 2) Maior sobreposição calculada por varredura, sem heap.
    let depth = scheduler::max_overlap(&agenda);

    println!(
        "\nSalas usadas (guloso): {} | LeetCode 2406: {} | maior sobreposição (sweep line): {}",
        groups.len(),
        lc_answer,
        depth
    );
    assert_eq!(groups.len(), lc_answer);
    assert_eq!(groups.len(), depth);
    println!("Conferência OK: o guloso é ótimo.");
}