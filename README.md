# Classic Algorithms in Rust

Implementing some classic algorithms in Rust for learning purposes:

| Algorithm                                                                                                                                        | Why is it interesting?         |
|--------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------|
| [Linear search](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/search/linear_search.rs)                    | $O(n)$                         |
| [Binary search](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/search/binary_search.rs)                    | $O(\mathrm{log}_2 n)$          |            | [Selection sort](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/sort/selection_sort.rs)                    | $O(n^2)$                       |           
| [Bubble sort](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/sort/bubble_sort.rs)                          | $O(n^2)$                       |
| [Quick sort](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/sort/quick_sort.rs)                            | $O(n \times \mathrm{log}_2 n)$ on average, $O(n^2)$ in the worst case | 
| [Counting sort](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/sort_and_search/src/sort/counting_sort.rs)                      | $O(n + k)$, where $n$ is the number of elements to be sorted and $k$ the range between the smallest an the largest elements |                   
| [Factorial](https://github.com/rheradio/classic-algorithms-in-rust/tree/main/recursion/src/factorial)                                             | Tail recursion                |                   
| [Fibonacci](https://github.com/rheradio/classic-algorithms-in-rust/tree/main/recursion/src/fibonacci)                                             | Basic recursion, dynamic programming (*memoization*) |                   
| [Knight's tour](https://github.com/rheradio/classic-algorithms-in-rust/blob/main/recursion/src/knights_tour/knights_tour.rs)                      | Recursion, backtracking, matrices & enums in Rust |
