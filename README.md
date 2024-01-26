# Classic Algorithms in Rust

Implementing some classic algorithms in Rust for learning purposes:

| Algorithm                                                                                                                                        | Why is it interesting?         |   Reference  |
|--------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------|--------------|
| [Linear search](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/search/linear_search.rs)                    | $O(n)$                         | [Stephens01] |
| [Binary search](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/search/binary_search.rs)                    | $O(\mathrm{log}_2 n)$          | [Stephens01] |
[Selection sort](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/sort/selection_sort.rs)                      | $O(n^2)$                         | [Stephens01] |        
| [Selection sort](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/sort/selection_sort.rs)                    | $O(n^2)$                       | [Bhargava]   |
| [Bubble sort](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/sort/bubble_sort.rs)                          | $O(n^2)$                       | [Stephens01] |
| [Quick sort](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/sort/quick_sort.rs)                            | $O(n \times \mathrm{log}_2 n)$ on average, $O(n^2)$ in the worst case | [Stephens01] |
| [Counting sort](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/sort/counting_sort.rs)                      | $O(n + k)$, where $n$ is the number of elements to be sorted and $k$ the range between the smallest an the largest elements | [Stephens01] |                  
| [Factorial](https://github.com/rheradio/classic-algorithms-in-rust/tree/main/recursion/src/factorial)                                            | Tail recursion                | [Stephens02] |                  
| [Fibonacci](https://github.com/rheradio/classic-algorithms-in-rust/tree/main/recursion/src/fibonacci)                                            | Basic recursion, dynamic programming (*memoization*) | [Stephens02] |                   
| [Knight's tour](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/recursion/src/knights_tour/knights_tour.rs)                     | Recursion, backtracking, matrices & enums in Rust | [Stephens02] | 
| [N-Queens](https://github.com/rheradio/classic-algorithms-in-rust/tree/main/recursion/src/n_queens)                                              | Recursion, backtracking, importance of solution space reduction | [Stephens02] | 
| [The Tower of Hanoi Puzzle](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/recursion/src/hanoi_tower/hanoi_tower.rs)           | Wild recursion | [Stephens02] |

## References: 

+ [Bhargava] Aditya Y. Bhargava. Grokking Algorithms, Second Edition. Manning, 2024.
+ [Stephens01] Rod Stephens. [Algorithm Projects with Rust: Sorting and Searching](https://www.manning.com/liveproject/sorting-and-searching-rust). Manning, 20023.
+ [Stephens02] Rod Stephens. [Algorithm Projects with Rust: Problem-Solving with Recursion](https://www.manning.com/liveproject/problem-solving-with-recursion-rust). Manning, 2023.
+ [Stephens03] Rod Stephens. [Algorithm Projects with Rust: Public Key Cryptography](https://www.manning.com/liveproject/public-key-cryptography-rust). Manning, 2023.
+ [Stephens04] Rod Stephens. [Algorithm Projects with Rust: Dynamic Programming](https://www.manning.com/liveproject/dynamic-programming-rust). Manning, 2023.
