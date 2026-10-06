# Interval Partitioning

**Número da Lista**: 1<br>
**Conteúdo da Disciplina**: Algoritmos Gulosos (Interval Partitioning e Min-Heap)<br>

## Alunos

| Matrícula | Aluno |
| -- | -- |
| 180113097 | Daniel Coimbra dos Santos |
| 180066161 | Luis Henrique Luz Costa |

## Sobre

Este projeto, escrito em Rust, resolve o problema de *interval partitioning*: dado um conjunto de intervalos, qual é o menor número de grupos necessário para que nenhum par de intervalos do mesmo grupo se sobreponha? Pensando em uma agenda, é o mesmo que perguntar de quantas salas precisamos para acomodar todas as reuniões.

O programa tem duas partes. A primeira é a solução do [LeetCode 2406 (Divide Intervals Into Minimum Number of Groups)](https://leetcode.com/problems/divide-intervals-into-minimum-number-of-groups/), de dificuldade média, que devolve apenas o número mínimo de grupos. A segunda aplica o mesmo algoritmo a uma agenda de reuniões, mostrando a decisão tomada para cada intervalo e desenhando a ocupação das salas em uma linha do tempo no terminal. O problema é equivalente ao Meeting Rooms II (LeetCode 253), que exige assinatura *premium*.

### Algoritmo

A estratégia é gulosa e usa uma min-heap:

1. Os intervalos são ordenados pelo início.
2. A heap guarda o horário em que cada sala fica livre, então o topo é sempre a sala que desocupa primeiro.
3. Para cada intervalo, se a sala do topo já estiver livre quando ele começa, ela é reaproveitada. Caso contrário, abre-se uma sala nova.
4. No final, o número de salas é o tamanho da heap.

A escolha gulosa se justifica facilmente: se a sala que desocupa mais cedo ainda está ocupada, todas as outras também estão, então abrir uma sala nova é inevitável.

Os extremos dos intervalos são inclusivos, por isso a comparação é estrita. Os intervalos `[1,5]` e `[5,10]` conflitam, pois ambos ocupam o instante 5, enquanto `[1,5]` e `[6,10]` não. A ordenação custa `O(n log n)` e cada operação na heap custa `O(log n)`, o que dá `O(n log n)` no total, com `O(n)` de espaço. Empates são resolvidos pelo índice do intervalo, então a saída é sempre a mesma para a mesma entrada.

Como conferência, o programa também calcula a maior quantidade de intervalos sobrepostos em um mesmo instante, por varredura (*sweep line*) e sem usar heap. Esse valor é um limite inferior para o número de salas e coincide com o resultado do guloso. A igualdade é verificada ao final de cada execução.

## Exemplo de execução

Com a agenda padrão, o programa mostra a decisão para cada intervalo e a ocupação final das salas:

```
Passos gulosos (ordenado pelo início):
  A [1,5] -> sala 0 (NOVA: nenhuma sala livre antes de 1)
  C [2,4] -> sala 1 (NOVA: nenhuma sala livre antes de 2)
  D [3,7] -> sala 2 (NOVA: nenhuma sala livre antes de 3)
  B [5,10] -> sala 1 (reaproveitada: ficou livre em 4)
  E [6,8] -> sala 0 (reaproveitada: ficou livre em 5)
  F [8,12] -> sala 2 (reaproveitada: ficou livre em 7)
  G [9,11] -> sala 0 (reaproveitada: ficou livre em 8)
  H [11,13] -> sala 1 (reaproveitada: ficou livre em 10)

Linha do tempo:
  tempo   1234567890123
  sala 0  AAAAAEEEGGG..
  sala 1  .CCCBBBBBBHHH
  sala 2  ..DDDDDFFFFF.

Salas usadas (guloso): 3 | LeetCode 2406: 3 | maior sobreposição (sweep line): 3
Conferência OK: o guloso é ótimo.
```

## Instalação

Linguagem: Rust (edição 2021)<br>
Framework: Não se aplica (utiliza apenas a biblioteca padrão, sem dependências externas)<br>

### Pré-requisitos
* Rust e Cargo instalados (via [rustup](https://rustup.rs))

### Como executar

Na raiz do projeto, execute com a agenda padrão:

```bash
cargo run
```

Também é possível passar intervalos próprios, no formato `inicio-fim`:

```bash
cargo run -- 1-5 5-10 2-4 3-7
```

Para rodar os testes (exemplos do LeetCode, extremos inclusivos e entrada vazia):

```bash
cargo test
```

### Estrutura dos Arquivos

```
.
├── Cargo.toml
├── README.md
└── src
    ├── leetcode.rs
    ├── main.rs
    └── scheduler.rs
```

* `src/leetcode.rs`: solução do LeetCode 2406 com min-heap, mais os testes.
* `src/scheduler.rs`: versão que registra a sala de cada intervalo, a varredura de conferência e a linha do tempo em texto.
* `src/main.rs`: programa principal, que executa as duas partes e confere os resultados.