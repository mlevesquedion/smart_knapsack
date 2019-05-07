# "Smart" Knapsack

A failed attempt to make the classic dynamic programming algorithm for the 0/1 knapsack problem faster by using a stack to avoid unnecessary computations.

## Rationale

The classic dynamic programming algorithm calculates some values which are not actually necessary to solve the problem. Most notably, every value in the last row except the last one is not necessary. Since this value can be computed using at most two values from the previous row, there are only two values in the before-last row that are actually necessary, and so on.

## Approach

The optimization attempt presented here stores coordinates of to-be computed values on a stack, simulating the recursion from the "brute force" version of the algorithm while avoiding unnecessary computations by using the table from the original algorithm. 

## Results

It is quite obvious from running both algorithms on a few large randomly-generated instances that the stack-based algorithm does almost as many computations as the original algorithm, while also being slower.

## Discussion

Except for very small instances, the stack adds a significant overhead and does not significantly reduce the number of computations. This failure is explained by the fact that it takes very few rows for the stack-based algorithm to reach a point from which it must do as many computations as the original algorithm, with extra overhead of using the stack. This is because the number of values to calculate increase roughly exponentially from one row to the previous, until the point where all values have to be calculated as in the original algorithm. Therefore, this point is quickly reached.
