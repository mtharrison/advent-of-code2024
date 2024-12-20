<h1 align="center">🦀 Advent of Code 2024 🎄</h1>

<div align="center">

<a href="">![build](https://github.com/mtharrison/advent-of-code2024/actions/workflows/rust.yml/badge.svg)</a>
<a href="">[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)</a>

</div>

My solutions to [AOC 2024](https://adventofcode.com/2024/about) in [The Rust Programming Language](https://www.rust-lang.org/).

<!---DAY1_BEGIN-->

### [⭐️⭐️ Day 1 - Historian Hysteria](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day01/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
In my solution, I utilized two `Vec<i32>` data structures for storing the location IDs from both lists. To calculate the total distance, I first sorted both vectors to ensure that the smallest numbers are paired correctly. After sorting, I iterated through the lists using a loop, calculating the absolute difference between the paired elements, accumulating the total distance. Additionally, I implemented a separate function to compute the similarity by counting occurrences of each number from the first list in the second list, multiplying each by its respective value, which allowed me to derive the total similarity efficiently. This approach leverages sorting for pairing and straightforward iteration for calculations, providing both clarity and performance.
</details>
            
<!---DAY1_END-->

<!---DAY2_BEGIN-->

### [⭐️⭐️ Day 2 - Red-Nosed Reports](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day02/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I used a recursive approach to determine the safety of reports based on their level sequences. The function <code>is_safe</code> checks adjacent differences in the levels, and if a difference exceeds the specified tolerance, it attempts to remove one of the offending elements and recurse to see if the resulting list is safe. This is facilitated using <code>cloned.remove(i)</code> to create modified versions of the report while preserving the original one for further checks. The <code>check_safe</code> function considers both increasing and decreasing sequences by reversing the report, allowing for a straightforward evaluation of both conditions. For the final counting, <code>count_safe</code> iterates through all reports, leveraging the results from <code>check_safe</code> to tally the number of safe reports efficiently. This combination of recursion and list manipulation allows handling variations in the levels while ensuring clarity and correctness in the solution's logic.
</details>
            
<!---DAY2_END-->

<!---DAY3_BEGIN-->

### [⭐️⭐️ Day 3 - Mull It Over](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day03/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I implemented a state machine approach for this problem, where the `Machine` struct manages the program's state, including an accumulator for summing multiplication results and a flag for enabling or disabling operations based on the presence of the `do` and `don't` instructions. I utilized a `Regex` to parse the corrupted input and extract valid `mul` instructions while ignoring invalid characters. The `scan_program` function processes the input string, generating a vector of `Instruction` enum variants, which the `Machine` then iterates over in the `run_program` method to perform calculations conditionally, thus accumulating the correct results. This approach efficiently separates parsing from execution logic, allowing clear handling of valid instructions.
</details>
            
<!---DAY3_END-->

<!---DAY4_BEGIN-->

### [⭐️⭐️ Day 4 - Ceres Search](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day04/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a `Grid<char>` structure to effectively represent the word search and facilitate both horizontal and vertical searches. I iterated through the rows, columns, and diagonals of the grid to gather all possible sequences that might contain the word "XMAS". For each of these sequences, I combined the characters into a string and used the `matches` method to count occurrences of "XMAS" and its overlaps, specifically "SAMX", in a concise manner. Additionally, for part 2, I examined 3x3 squares within the grid, leveraging the `is_xmas_grid` function to check for configurations that contained the word "XMAS". This method ensured an efficient and clear counting process across all relevant orientations.
</details>
            
<!---DAY4_END-->

<!---DAY5_BEGIN-->

### [⭐️⭐️ Day 5 - Print Queue](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day05/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
My approach to solving the puzzle involves parsing the input into a vector of page ordering rules represented as edges and a vector of update paths. To determine if each path complies with the rules, I utilize a linear search in the `valid_path` function, checking consecutive pages against the relevant rules. For paths deemed invalid, I extract the relevant ordering rules using `pick_edges`, allowing me to create a directed acyclic graph (DAG) from the subset of rules that apply. I then apply topological sorting on this graph to deduce the correct printing order, enabling me to identify the middle page number for each correctly ordered update. This structured handling of edges and paths through a combination of linear searches and graph algorithms ensures efficient validation and sorting of the updates.
</details>
            
<!---DAY5_END-->

<!---DAY6_BEGIN-->

### [⭐️⭐️ Day 6 - Guard Gallivant](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day06/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
To solve the puzzle, I modeled the guard's environment using a grid represented by a 2D vector of cells, each with distinct states: guard, vacant, obstacle, or visited. I utilized a `GuardState` struct to track the guard's position and direction, coupled with a `World` struct to encapsulate the grid and the guard's state. For tracking the distinct positions visited by the guard efficiently, I employed `HashSet` from the `ahash` crate, which supports fast insertion and containment checks. The primary algorithm iteratively simulated the guard's movement based on the rules provided, updating the state of each cell and redirecting the guard as necessary until it either exits the grid or encounters a loop, thus allowing me to count and return the total unique positions visited. The `step` method encapsulated the logic for movement, and I systematically placed obstacles for additional checks in the second part using parallel iteration for efficiency.
</details>
            
<!---DAY6_END-->

<!---DAY7_BEGIN-->

### [⭐️⭐️ Day 7 - Bridge Repair](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day07/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a combinatorial approach to explore all possible operator insertions between operands in the equations, using the `multi_cartesian_product` method from the `itertools` crate to generate combinations of the operators '+' and '*' (and additionally '|' in part two). The input equations were structured as tuples containing the expected result and a vector of operands. By parallelizing the evaluation of these combinations with Rayon, I processed each equation concurrently, evaluating the expression defined by the operands and operators in a left-to-right manner. The sum of all matching results was then accumulated, leveraging Rust's efficient handling of concurrent operations to improve performance on larger datasets. Final results were verified through test cases ensuring correctness across both example and real input scenarios.
</details>
            
<!---DAY7_END-->

<!---DAY8_BEGIN-->

### [⭐️⭐️ Day 8 - Resonant Collinearity](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day08/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I used the `CellTower` struct to represent each antenna, storing its position and frequency, which allowed for easy comparisons and manipulation. By leveraging the `itertools` crate, I efficiently generated combinations of antennas to identify pairs with matching frequencies, ensuring I only processed valid pairs. For finding antinodes, I computed the directional vectors for each pair and iteratively generated potential antinode positions while checking their validity against the grid bounds with the `out_of_bounds` function. Finally, I utilized a combination of `flat_map` and `filter` to accumulate unique antinode positions, counting them with the `antinodes_unique` function, which effectively handled duplicates. This approach ensured the solution was optimized for both clarity and performance.
</details>
            
<!---DAY8_END-->

<!---DAY9_BEGIN-->

### [⭐️⭐️ Day 9 - Disk Fragmenter](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day09/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a custom data structure to represent the disk blocks, distinguishing between free spaces and file segments using an `enum` called `DiskBlock`. The core algorithm for defragmentation involved two strategies: a simple pass that moved file blocks left until all free spaces were consolidated, and a more complex approach that involved tracking file segments and free regions. I maintained two vectors, one for files and another for free spaces, to facilitate efficient adjustments during swaps, minimizing unnecessary computations. For checksum calculation, I implemented a straightforward iteration through the blocks, applying the required multiplications and summing up the values conditionally based on the block type. The overall design optimized space operations while ensuring clarity through structured types.
</details>
            
<!---DAY9_END-->

<!---DAY10_BEGIN-->

### [⭐️⭐️ Day 10 - Hoof It](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day10/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
To solve the puzzle, I used a grid data structure to represent the topographic map, and implemented depth-first search (DFS) algorithms to explore potential hiking trails starting from trailheads marked by height `0`. I created two separate functions: `find_trails` to gather the reachable positions with a height of `9`, storing them in a `HashSet` to avoid duplicates, and `distinct_trails` to simply count paths to the `9` heights without storing them. The `trailhead_score` and `trailhead_rating` functions check each position for trailheads, and I utilized a higher-order function, `map_score`, to iterate over the grid and apply the scoring functions, summing the scores of all trailheads to yield the final result. This approach efficiently calculates the scores by leveraging recursion and the properties of the grid, ensuring that I respect the puzzle's constraints on movement and trail formation.
</details>
            
<!---DAY10_END-->

<!---DAY11_BEGIN-->

### [⭐️⭐️ Day 11 - Plutonian Pebbles](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day11/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a recursive approach combined with memoization to efficiently compute the number of stones after a specified number of blinks. The key data structure employed was a `HashMap` to store previously computed results based on the current stone and the number of blinks remaining, which allows for skipping redundant calculations and significantly reduces the time complexity. For each stone, I implemented three distinct transformation rules: converting `0` to `1`, splitting even-numbered digits into two new stones, and multiplying the stone's value by `2024` if no other rules apply. The `blink_recursive_count` function manages the recursive descent through the stone transformations while utilizing the `apply_rules` function to determine the next state of each stone, ensuring the preservation of the order throughout the transformation process. This structured approach provides a clear, efficient pathway to solving the problem at scale.
</details>
            
<!---DAY11_END-->

<!---DAY12_BEGIN-->

### [⭐️⭐️ Day 12 - Garden Groups](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day12/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
To tackle the problem, I utilized a depth-first search (DFS) approach to effectively identify distinct regions of garden plots in the grid. Each region is defined by a set of connected plots with the same plant type, which I tracked using a `HashSet` for visited plots to avoid duplicates. For each unvisited plot, I initialized a DFS that collected both the total number of plots in the region and the edges contributing to the perimeter. I implemented the edges as a `HashSet` of tuples representing boundary sides, where I carefully deduplicated edges by considering neighboring plots and whether they belonged to the same region. Finally, I computed the price of the fence for each region using two separate strategies: one based on the total number of edges and another based on the number of sides, which I calculated by analyzing the vertices and right-angle turns formed by the perimeter. This holistic approach ensured that I could accurately evaluate the total price for fencing all identified regions in the garden plot grid.
</details>
            
<!---DAY12_END-->

<!---DAY13_BEGIN-->

### [⭐️⭐️ Day 13 - Claw Contraption](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day13/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I implemented a solution that models each claw machine's behavior using a `ClawMachine` struct, storing the movement values for buttons A and B, as well as the prize location. The core of the solution involves solving a system of linear equations created by equating the movements caused by button presses to the exact prize coordinates. To do this, I employed integer arithmetic to ensure that calculations determine feasible button press counts (`a` for button A and `b` for button B), while considering their associated costs. The `cost` method computes the total cost based on valid movements, while the `solve` method uses coefficients derived from the equations to check if a solution exists, returning None if the conditions are not met. The results for all machines are summed up to provide the total minimum token cost needed to win as many prizes as possible.
</details>
            
<!---DAY13_END-->

<!---DAY14_BEGIN-->

### [⭐️⭐️ Day 14 - Restroom Redoubt](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day14/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a `Robot` struct to encapsulate each robot's position and velocity, and a `Map` struct to manage the collection of robots and the simulation of their movements on a defined grid. I employed a stepping mechanism that updates the robot positions based on their velocities while incorporating a wrap-around logic for boundary conditions using modular arithmetic. After running the simulation for a specified number of steps (100 seconds), I counted the robots in each of the four quadrants of the grid and computed the safety factor by multiplying these counts. The grid was also visually represented using a `Grid<char>` structure, allowing for additional functionality to analyze the pattern of robots. This approach facilitated efficient simulation and straightforward calculations for the final result.
</details>
            
<!---DAY14_END-->

<!---DAY15_BEGIN-->

### [⭐️⭐️ Day 15 - Warehouse Woes](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day15/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
To solve this puzzle, I designed a `Warehouse` struct, which includes a grid representation of the warehouse using a custom `Grid` type to store different types of cells (walls, vacant spaces, boxes, and the robot) effectively. The approach relies on recursion to identify and manage the boxes that can be pushed by the robot based on its movement direction, using depth-first search techniques to explore and aggregate contiguous boxes in the direction of the push. I implemented separate functions for moving boxes vertically and horizontally to handle different movement constraints, ensuring that boxes cannot be pushed into walls. The final part of the solution involves calculating the GPS coordinates for the boxes post-movement, summing these to output the desired result. Additionally, I included an upscaling function to double the size of the grid when necessary, reflecting the need to manage box configurations accurately.
</details>
            
<!---DAY15_END-->

<!---DAY16_BEGIN-->

### [👨‍💻👨‍💻 Day 16 - Reindeer Maze](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day16/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>

</details>
            
<!---DAY16_END-->

<!---DAY17_BEGIN-->

### [👨‍💻👨‍💻 Day 17 - Chronospatial Computer](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day17/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>

</details>
            
<!---DAY17_END-->

<!---DAY18_BEGIN-->

### [👨‍💻👨‍💻 Day 18 - RAM Run](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day18/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>

</details>
            
<!---DAY18_END-->

<!---DAY19_BEGIN-->

### [⭐️⭐️ Day 19 - Linen Layout](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day19/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized dynamic programming to efficiently determine the number of ways each design can be constructed from the available towel patterns. By employing a memoization technique with a vector `memo`, where `memo[i]` represents the number of ways to construct the substring of the design up to index `i`, I initialized it such that `memo[0]` is set to 1 (the base case). I then utilized a `HashSet` to store the available patterns for O(1) average-time complexity lookups. For each index in the design, I inspected all possible previous substrings, updating the memoization vector based on whether the current substring exists in the pattern set. This approach ensures that we efficiently count valid combinations while minimizing redundant calculations.
</details>
            
<!---DAY19_END-->

<!---DAY20_BEGIN-->

### [👨‍💻👨‍💻 Day 20 - Race Condition](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day20/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>

</details>
            
<!---DAY20_END-->

<!---DAY21_BEGIN-->

<!---DAY21_END-->

<!---DAY22_BEGIN-->

<!---DAY22_END-->

<!---DAY23_BEGIN-->

<!---DAY23_END-->

<!---DAY24_BEGIN-->

<!---DAY24_END-->

<!---DAY25_BEGIN-->

<!---DAY25_END-->
