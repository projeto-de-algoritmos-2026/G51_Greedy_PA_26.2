//! Versão "com atribuição" do interval partitioning, usada na demonstração.
//!
//! A solução do LeetCode (`leetcode.rs`) só devolve QUANTOS grupos são
//! necessários. Aqui o mesmo algoritmo guloso também registra QUAL
//! intervalo foi para QUAL grupo, o que permite desenhar uma linha do tempo.
//! Também há uma verificação independente: o número mínimo de grupos é
//! sempre igual à maior quantidade de intervalos sobrepostos num mesmo
//! instante (a "profundidade" dos intervalos).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Intervalo fechado (início, fim), ambos inclusivos.
pub type Interval = (i32, i32);

/// Letra usada para identificar o intervalo `index` (A, B, C, ...).
pub fn id_of(index: usize) -> char {
    (b'A' + (index % 26) as u8) as char
}

/// Divide os intervalos em grupos usando o algoritmo guloso com min-heap.
/// Retorna, para cada grupo, os índices dos intervalos atribuídos a ele.
pub fn partition(intervals: &[Interval], verbose: bool) -> Vec<Vec<usize>> {
    // Ordena os ÍNDICES pelo início (desempate por fim e índice, para que
    // a saída seja sempre a mesma).
    let mut order: Vec<usize> = (0..intervals.len()).collect();
    order.sort_by_key(|&i| (intervals[i].0, intervals[i].1, i));

    // Min-heap de (horário em que o grupo fica livre, id do grupo).
    // O id no segundo campo serve de desempate determinístico.
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();

    for idx in order {
        let (start, end) = intervals[idx];

        // O grupo que termina mais cedo está livre? (fim < início, estrito)
        let reused = match heap.peek() {
            Some(&Reverse((free_at, g))) if free_at < start => {
                heap.pop();
                Some((free_at, g))
            }
            _ => None,
        };

        // Reaproveita o grupo ou cria um novo.
        let group = match reused {
            Some((_, g)) => g,
            None => {
                groups.push(Vec::new());
                groups.len() - 1
            }
        };

        if verbose {
            match reused {
                Some((free_at, g)) => println!(
                    "  {} [{},{}] -> sala {} (reaproveitada: ficou livre em {})",
                    id_of(idx), start, end, g, free_at
                ),
                None => println!(
                    "  {} [{},{}] -> sala {} (NOVA: nenhuma sala livre antes de {})",
                    id_of(idx), start, end, group, start
                ),
            }
        }

        groups[group].push(idx);
        // A sala agora só fica livre depois do fim deste intervalo.
        heap.push(Reverse((end, group)));
    }
    groups
}

/// Maior número de intervalos sobrepostos em algum instante, calculado por
/// varredura (sweep line), sem usar heap. Serve para conferir o resultado
/// guloso: os dois valores precisam ser iguais.
pub fn max_overlap(intervals: &[Interval]) -> usize {
    // Eventos (instante, +1 = começa / -1 = termina). Como o fim é
    // inclusivo, o intervalo deixa de contar em `fim + 1`. Ordenando as
    // tuplas, -1 vem antes de +1 no mesmo instante: quem termina em t-1
    // libera a vaga antes de quem começa em t.
    let mut events: Vec<(i32, i32)> = Vec::new();
    for &(s, e) in intervals {
        events.push((s, 1));
        events.push((e + 1, -1));
    }
    events.sort();

    let (mut current, mut best) = (0i32, 0i32);
    for (_, delta) in events {
        current += delta;
        best = best.max(current);
    }
    best as usize
}

/// Desenha uma linha do tempo em texto: uma linha por sala, uma coluna por
/// unidade de tempo, letra do intervalo onde a sala está ocupada.
pub fn print_gantt(intervals: &[Interval], groups: &[Vec<usize>]) {
    if intervals.is_empty() {
        return;
    }
    let lo = intervals.iter().map(|iv| iv.0).min().unwrap();
    let hi = intervals.iter().map(|iv| iv.1).max().unwrap();
    if hi - lo > 60 {
        println!("(linha do tempo grande demais; gráfico omitido)");
        return;
    }

    // Régua de tempo (último dígito de cada instante).
    print!("{:<10}", "  tempo");
    for t in lo..=hi {
        print!("{}", t.rem_euclid(10));
    }
    println!();

    for (g, members) in groups.iter().enumerate() {
        print!("  sala {:<2} ", g);
        for t in lo..=hi {
            // Algum intervalo desta sala cobre o instante t?
            let c = members
                .iter()
                .find(|&&i| intervals[i].0 <= t && t <= intervals[i].1)
                .map(|&i| id_of(i))
                .unwrap_or('.');
            print!("{}", c);
        }
        println!();
    }
}