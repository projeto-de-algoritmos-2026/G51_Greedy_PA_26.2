//! LeetCode 2406 — Divide Intervals Into Minimum Number of Groups (Medium)
//! https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/
//!
//! Enunciado: dado um vetor `intervals`, onde `intervals[i] = [início, fim]`
//! (ambos INCLUSIVOS), divida os intervalos em grupos de modo que nenhum par
//! de intervalos do mesmo grupo se intersecte. Retorne o MENOR número de
//! grupos possível. Dois intervalos se intersectam se compartilham ao menos
//! um número: [1,5] e [5,10] se intersectam, mas [1,5] e [6,10] não.
//!
//! É o problema clássico de "interval partitioning" (o mesmo do Meeting
//! Rooms II, LeetCode 253): quantas salas são necessárias para todas as
//! reuniões?
//!
//! Ideia (algoritmo guloso):
//!   1. Ordene os intervalos pelo início.
//!   2. Mantenha uma MIN-HEAP com o horário de término de cada grupo
//!      (o topo é o grupo que "termina mais cedo").
//!   3. Para cada intervalo, em ordem de início:
//!        - se o grupo que termina mais cedo já terminou ANTES do início do
//!          intervalo (fim < início), reaproveite esse grupo;
//!        - caso contrário, nenhum grupo serve: crie um grupo novo.
//!   4. A resposta é o tamanho final da heap (= número de grupos).
//!
//! Por que funciona: se nem o grupo que termina mais cedo está livre, então
//! nenhum outro está. Logo, criar um grupo novo é inevitável.
//!
//! Complexidade: O(n log n) de tempo (ordenação + operações na heap) e
//! O(n) de espaço.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Solução "limpa", com a assinatura esperada pelo LeetCode.
pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
    solve(intervals, false)
}

/// Mesma solução, mas imprime cada decisão gulosa (usada na demonstração).
pub fn min_groups_verbose(intervals: Vec<Vec<i32>>) -> i32 {
    solve(intervals, true)
}

/// Implementação compartilhada. `trace` liga/desliga a impressão dos passos.
fn solve(mut intervals: Vec<Vec<i32>>, trace: bool) -> i32 {
    // Passo 1: ordena pelo início do intervalo.
    intervals.sort_unstable_by_key(|iv| iv[0]);

    // `BinaryHeap` do Rust é uma MAX-heap; `Reverse` a transforma em
    // MIN-heap. Cada elemento é o horário de término de um grupo.
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for iv in &intervals {
        let (start, end) = (iv[0], iv[1]);

        // Passo 3: o grupo que termina mais cedo já acabou antes de `start`?
        // A comparação é ESTRITA (<) porque os extremos são inclusivos:
        // um grupo que termina em 5 NÃO pode receber um intervalo que
        // começa em 5.
        let reused = match heap.peek() {
            Some(&Reverse(earliest_end)) if earliest_end < start => {
                heap.pop(); // libera o grupo; ele será reinserido abaixo
                Some(earliest_end)
            }
            _ => None,
        };

        // O grupo (novo ou reaproveitado) passa a terminar em `end`.
        heap.push(Reverse(end));

        if trace {
            match reused {
                Some(e) => println!(
                    "  [{},{}] -> reaproveita grupo que terminava em {}  (grupos: {})",
                    start, end, e, heap.len()
                ),
                None => println!(
                    "  [{},{}] -> grupo NOVO, nenhum livre antes de {}  (grupos: {})",
                    start, end, start, heap.len()
                ),
            }
        }
    }

    // Cada elemento da heap é um grupo.
    heap.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    // Exemplo 1 do LeetCode.
    #[test]
    fn example_1() {
        let v = vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]];
        assert_eq!(min_groups(v), 3);
    }

    // Exemplo 2 do LeetCode: nenhum par se intersecta.
    #[test]
    fn example_2() {
        let v = vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]];
        assert_eq!(min_groups(v), 1);
    }

    // Extremos inclusivos: [1,5] e [5,10] se intersectam em 5.
    #[test]
    fn touching_intervals_conflict() {
        assert_eq!(min_groups(vec![vec![1, 5], vec![5, 10]]), 2);
        assert_eq!(min_groups(vec![vec![1, 5], vec![6, 10]]), 1);
    }

    // Entrada vazia: nenhum grupo necessário.
    #[test]
    fn empty() {
        assert_eq!(min_groups(vec![]), 0);
    }
}