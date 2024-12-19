<h1 align="center">🦀 Advent of Code 2024 🎄</h1>

<div align="center">

<a href="">![build](https://github.com/mtharrison/advent-of-code2024/actions/workflows/rust.yml/badge.svg)</a>
<a href="">[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)</a>

</div>

My solutions to [AOC 2024](https://adventofcode.com/2024/about) in [The Rust Programming Language](https://www.rust-lang.org/).

<!---DAY1_BEGIN-->
## [⭐️⭐️ Day 1 - Historian Hysteria](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day01/mod.rs)

To solve the puzzle, I employed vectors to store location IDs for both lists and utilized Rust's efficient sorting algorithms. By sorting both vectors, I ensured that corresponding elements could be directly paired based on their indices to compute the total distance using the absolute difference of each pair. This straightforward approach simplifies the calculations and allows for linear traversal after sorting. For the similarity calculation, I iterated through the first list and employed a filter on the second list to count occurrences, which allows for straightforward aggregation of the similarity score. Overall, this method efficiently combines sorting, traversal, and filtering operations to derive the necessary results.


<!---DAY1_END-->

<!---DAY2_BEGIN-->
## [⭐️⭐️ Day 2 - Red-Nosed Reports](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day02/mod.rs)

I used a recursive approach to validate the safety of each report, leveraging Rust's vector data structure to store and manipulate the levels. In `is_safe`, I traverse the report to calculate the differences between adjacent levels, ensuring they lie within the specified range of 1 to 3. If a difference exceeds this range and tolerance is enabled, I create two clones of the report excluding the problematic elements to reevaluate the remaining levels, allowing for potential safety by omitting outliers. The `check_safe` function checks both the original and reversed versions of the report to handle both increasing and decreasing sequences, while `count_safe` aggregates results from each report. This approach efficiently combines linear scans with recursion to handle possible outlier cases in reports.


<!---DAY2_END-->

<!---DAY3_BEGIN-->
## [⭐️⭐️ Day 3 - Mull It Over](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day03/mod.rs)

I chose to leverage a combination of a state machine pattern and regular expressions to parse and evaluate the instructions from the corrupted input. I defined an `Instruction` enum to encapsulate the possible operations, specifically focusing on multiplication, while implementing a `Machine` struct to maintain the state and perform computations. The regex is employed to extract valid instructions, ignoring any malformed entries, which I then map into the corresponding `Instruction` variants. During execution, my state machine accumulates results based on valid multiplications, and the use of booleans allows me to conditionally enable or disable processing of "do" and "don't" instructions. This modular and structured approach ensures clarity and maintainability while effectively solving the puzzle.




<!---DAY3_END-->

<!---DAY4_BEGIN-->
## [⭐️⭐️ Day 4 - Ceres Search](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day04/mod.rs)

To tackle the problem, I utilized a `Grid` data structure to represent the letter matrix, enabling efficient access to individual characters based on their coordinates. During the search, I extracted rows, columns, and diagonal sequences from the grid for examination. By iterating over these sequences, I captured them as strings and leveraged the `matches` method to count occurrences of the target word "XMAS" and its variants. This method allowed for clear and concise pattern matching without manually checking each character, streamlining the detection and ensuring all variations were accounted for efficiently. Additionally, for the second part of the puzzle, I implemented a function that checks 3x3 subgrids to identify valid configurations of the word, maintaining clarity and efficiency in the code structure.


<!---DAY4_END-->

<!---DAY5_BEGIN-->
## [⭐️⭐️ Day 5 - Print Queue](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day05/mod.rs)

I utilized a combination of vectors and tuples to represent the directed page ordering rules and the paths. The `valid_path` function checks whether a given path adheres to the provided rules by iterating through the path and matching the orders against the rules vector. For paths violating the rules, I extracted relevant edges through the `pick_edges` function, which filters the rules down to only those applicable to the current path. This allowed me to construct a directed acyclic graph (DAG) from the relevant edges, enabling me to determine the correct order of pages utilizing topological sorting. By calculating the middle page for both valid and invalid paths and accumulating these values, I efficiently derived the final result.


<!---DAY5_END-->

<!---DAY6_BEGIN-->
## [⭐️⭐️ Day 6 - Guard Gallivant](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day06/mod.rs)

I utilized a 2D vector to represent the game world, where each cell can be a guard, vacant, obstacle, or visited, enabling efficient state tracking. The guard's movement is handled through a state machine approach within the `step` method, where I calculate the new position based on the current direction and check for obstructions. Upon encountering an obstacle, the guard turns right by updating its direction, and I track visited positions using a `HashSet` to ensure unique counts. This combination of structures allows me to efficiently manage movement and detect looping states while maintaining clarity and performance, crucial for solving both parts of the puzzle effectively.


<!---DAY6_END-->

<!---DAY7_BEGIN-->
## [⭐️⭐️ Day 7 - Bridge Repair](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day07/mod.rs)

I used a parallelized approach with the `rayon` library to efficiently evaluate multiple operator combinations for each equation in the input. I represented each equation as a tuple containing a test value and a vector of operands. To explore all combinations of operators (`+`, `*`, and a custom `|`), I employed the `multi_cartesian_product` function from the `itertools` crate, allowing me to generate all possible arrangements of operators. For each combination, I computed the result by sequentially applying the operators from left to right, comparing the final value to the expected result. Valid sums from true equations were then accumulated to produce the final total calibration result. This structure not only streamlined the process of testing each equation but also significantly improved performance by leveraging parallel computation.


<!---DAY7_END-->

<!---DAY8_BEGIN-->
## [⭐️⭐️ Day 8 - Resonant Collinearity](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day08/mod.rs)

I utilized a combination of structs and iterator methods to efficiently identify unique antinode locations. First, I defined a `CellTower` struct to hold the position and frequency of each antenna. By leveraging `itertools::combinations`, I generated all pairs of antennas to check for matching frequencies, allowing for targeted calculations of potential antinode positions using vector arithmetic. For the first part, I computed two specific antinodes for each pair by determining their directional distances; in the second part, I extended this approach to iteratively find all valid antinode positions along each direction until reaching the bounds of the defined grid. I ensured antinode uniqueness by filtering and collecting results in a `Vec`, using a simple method that counted distinct entries using `iter().unique()`. This encapsulated the logic required while maintaining clarity and efficiency in both computation and memory management.


<!---DAY8_END-->

<!---DAY9_BEGIN-->
## [⭐️⭐️ Day 9 - Disk Fragmenter](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day09/mod.rs)

I used a custom data structure to represent the disk layout, where `DiskBlock` enum distinguishes between free space and file blocks, which are stored in a vector for efficient indexed access. The `Disk` struct maintains the blocks along with indices to track the first free position and the last block. For defragmentation, I implemented two methods: `defrag_simple` uses a two-pointer approach to compact files from the right to the left while maintaining the order, and `defrag_files` identifies free regions to relocate files based on their sizes and positions. The checksum is computed by iterating through the blocks and aggregating the products of positions and file IDs, ensuring efficient space and time complexity performance while handling the operations.


<!---DAY9_END-->

<!---DAY10_BEGIN-->
## [⭐️⭐️ Day 10 - Hoof It](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day10/mod.rs)

I implemented a depth-first search (DFS) approach to traverse the topographic map, starting from each trailhead (position with height '0'). I utilized a recursive function, `find_trails`, which explores all neighboring positions that maintain the required height progression, effectively tracking reachable positions with a `HashSet`. To compute the score for each trailhead, I also created a separate function, `distinct_trails`, which counts the number of achievable heights of '9' through similar recursive exploration. This DFS approach efficiently captures the necessary paths with incremental height changes while leveraging a `Grid` data structure for managing the map and accessing neighbors. The overall score is aggregated by iterating over each grid position and applying the respective scoring function.


<!---DAY10_END-->

<!---DAY11_BEGIN-->
## [⭐️⭐️ Day 11 - Plutonian Pebbles](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day11/mod.rs)

I utilized a recursive approach to model the behavior of the stones with efficient memoization to optimize repeated calculations. Each stone undergoes transformation based on a set of rules encapsulated in distinct functions, where I first check for a specific rule to apply and then split or multiply the stone's value as needed. I used a `HashMap` to cache results of transformations for specific stone values and the number of blinks remaining, allowing me to avoid redundant computations and significantly reduce the time complexity. The recursive function processes each stone, splits the input vector, and accumulates the total count of stones produced after a given number of blinks, maintaining the order of transformations.


<!---DAY11_END-->

<!---DAY12_BEGIN-->
## [⭐️⭐️ Day 12 - Garden Groups](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day12/mod.rs)

I utilized a depth-first search (DFS) approach to explore the garden grid, identifying regions of plants by tracking visited plots in a `HashSet`, which enables efficient lookups and ensures each plot is only processed once. For each unvisited plot, I recorded the number of plots in the region and collected edges representing its perimeter using a `HashSet` to eliminate duplicates. I then calculated the total price of fencing each region by multiplying the area (number of plots) by the number of unique edges. Additionally, I implemented a secondary method to compute the price based on sides, leveraging the relationship between edges and the number of right angles, ensuring that I accurately counted edges contributing to the overall perimeter without redundancies.


<!---DAY12_END-->

<!---DAY13_BEGIN-->
## [⭐️⭐️ Day 13 - Claw Contraption](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day13/mod.rs)

I implemented a solution that models each claw machine as a struct containing movement coefficients for buttons A and B as well as the target prize location. To determine the number of times each button should be pressed, I formulated the problem as a system of linear equations, where I derived the equations based on the coefficients of movements and the target coordinates. By utilizing integer arithmetic to solve for the number of presses, I checked for viable solutions and computed the total cost in tokens using pre-defined costs for each button. This allows me to iterate through all machines to find the minimum token expenditure needed to win as many prizes as possible by leveraging algebraic manipulation.


<!---DAY13_END-->

<!---DAY14_BEGIN-->
## [⭐️⭐️ Day 14 - Restroom Redoubt](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day14/mod.rs)

In my approach, I utilized a `Map` struct to represent the environment containing the robots, encapsulating their positions and velocities, and handling the simulation of their movement. For efficient tracking of the robot positions, I made use of a `Vec2d` struct to handle coordinate operations, including wrapping around the edges of the map during movement, which was implemented through modular arithmetic. The `step_n_times` method allows for simulating the robot movements over a specified number of time steps, while the `count_quadrants` method tallies the number of robots present in each quadrant of the grid after the simulation. I also incorporated a `Grid` structure to visualize the robot positions, providing a convenient method to format the output. The safety factor is calculated by multiplying the counts of robots in each quadrant, thereby determining the safest area. Overall, the choice of data structures supported clear organization and efficient manipulation of the robot states.


<!---DAY14_END-->

<!---DAY15_BEGIN-->
## [⭐️⭐️ Day 15 - Warehouse Woes](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day15/mod.rs)

I implemented the solution using a grid-based approach, where the `Warehouse` struct encapsulates the grid containing different cell types, including walls, vacancies, boxes (which are represented as either single or bipartite), and the robot's position. To handle movement and interactions, I defined methods for pushing boxes in both vertical and horizontal directions, relying on depth-first traversal to gather pushable boxes and manage their movement while avoiding walls. The robot's movements are validated against the grid, ensuring we update the state only when valid actions can be performed. After processing the instructions, I calculate the GPS score by iterating through the grid and computing the coordinates for each box, summed up to provide the final score. This approach effectively combines grid manipulation with spatial logic to simulate the problem's requirements accurately.


<!---DAY15_END-->

<!---DAY16_BEGIN-->
## [👨‍💻👨‍💻 Day 16 - Reindeer Maze](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day16/mod.rs)




<!---DAY16_END-->

<!---DAY17_BEGIN-->
## [👨‍💻👨‍💻 Day 17 - Chronospatial Computer](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day17/mod.rs)




<!---DAY17_END-->

<!---DAY18_BEGIN-->
## [👨‍💻👨‍💻 Day 18 - RAM Run](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day18/mod.rs)




<!---DAY18_END-->

<!---DAY19_BEGIN-->
## [⭐️⭐️ Day 19 - Linen Layout](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day19/mod.rs)

I implemented a dynamic programming approach to solve the problem of determining the number of ways to construct desired towel designs using available patterns. I utilized a vector, `memo`, to keep track of the number of ways to construct substrings of the design, where `memo[i]` represents the number of ways to construct the first `i` characters of the design string. I iterated through the design string, checking each substring against a `HashSet` of available patterns for efficient membership testing. For each valid substring found, I updated the `memo` table by adding the number of ways to construct the preceding segment, effectively building upon previously computed results. This methodology ensures an efficient calculation of possible ways to form each design without redundant checks.


<!---DAY19_END-->

<!---DAY20_BEGIN-->

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
