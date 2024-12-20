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

To solve the puzzle, I utilized two vectors to store the pairs of location IDs from the input data, which allowed for efficient indexing and sorting. After parsing the input, I sorted both vectors in ascending order to facilitate the pairing of corresponding elements. The `get_distance` function calculates the total absolute difference between paired elements in the sorted lists, ensuring that the smallest values are compared directly, thus providing the correct metric of distance. Additionally, I implemented a `get_similarity` function that counts how many times each number appears in the second list, leveraging iterators and filters for efficient calculations. The solution ensures optimal performance by minimizing unnecessary computations and using direct indexing on sorted vectors.
</details>
            
<!---DAY1_END-->

<!---DAY2_BEGIN-->

### [⭐️⭐️ Day 2 - Red-Nosed Reports](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day02/mod.rs)
            
<details>
<summary>See explanation</summary>

To solve the problem of determining safe reports, I employed a recursive approach using a helper function, `is_safe`, which checks the conditions for each report. I utilized vectors to represent each report's levels, where the `check_safe` function assesses both the original order and the reversed order of levels, allowing for simplicity in evaluating increasing and decreasing sequences. Additionally, I implemented tolerance to account for removable adjacent levels when a sequence is found to be unsafe, which is facilitated by recursively removing elements and rechecking the conditions. This combination of recursion, vector manipulation, and clear separation of concerns in functions ensures efficient checking of the reports against the specified safety criteria.
</details>
            
<!---DAY2_END-->

<!---DAY3_BEGIN-->

### [⭐️⭐️ Day 3 - Mull It Over](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day03/mod.rs)
            
<details>
<summary>See explanation</summary>

I utilized a combination of regular expressions and a simple state machine to process the corrupted instructions from the input. A `Regex` pattern was defined to match valid `mul(X,Y)` instructions while ignoring any invalid formats. The state machine class, `Machine`, maintained an `accumulator` to store the cumulative result of valid multiplications, along with a toggle for enabling or disabling instruction evaluation. As I scanned through the parsed instructions, I instantiated `Instruction` enums representing the operations and efficiently computed the total using the accumulated results. This modular approach allowed for clear separation of concerns, enhancing both readability and maintainability of the code.
</details>
            
<!---DAY3_END-->

<!---DAY4_BEGIN-->

### [⭐️⭐️ Day 4 - Ceres Search](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day04/mod.rs)
            
<details>
<summary>See explanation</summary>

I approached the problem by utilizing a custom `Grid` data structure to efficiently represent the character matrix of the puzzle. I first collected potential search directions—rows, columns, and diagonals—into a single iterable, allowing me to streamline the search process. For each direction's string representation, I leveraged Rust's `matches` method to count occurrences of the target word "XMAS" and its variations, thereby optimizing the search with minimal overhead. Additionally, I implemented a helper function, `is_xmas_grid`, to check 3x3 squares for the presence of desired patterns, enhancing modularity and code readability. This structured method ensured an effective solution while maintaining clarity in the overall implementation.
</details>
            
<!---DAY4_END-->

<!---DAY5_BEGIN-->

### [⭐️⭐️ Day 5 - Print Queue](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day05/mod.rs)
            
<details>
<summary>See explanation</summary>

I first parsed the input into two structures: a vector of ordering rules (edges) and a vector of paths to be validated. To determine if a path complies with the rules, I implemented a function (`valid_path`) that checks if each consecutive pair of pages in the path respects the defined ordering. For paths that do not adhere to the rules, I collected the relevant edges for that path and constructed a graph, allowing me to perform a topological sort to find a valid order of pages. This ensures that I can efficiently determine the middle page of each valid path. Finally, I accumulated the middle page numbers from all correctly ordered updates to obtain the solution.
</details>
            
<!---DAY5_END-->

<!---DAY6_BEGIN-->

### [⭐️⭐️ Day 6 - Guard Gallivant](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day06/mod.rs)
            
<details>
<summary>See explanation</summary>

In my solution, I modeled the guard's environment using a `World` struct that contains a grid of `Cell` enums to represent various states (obstacle, vacant, visited, and guard). The guard's position and movement direction are tracked using a `GuardState` struct. I implemented movement logic based on the guard's defined behavior: if there's no obstacle ahead, the guard moves forward; otherwise, it turns right. To efficiently track visited positions, I utilized `HashSet` from the `ahash` crate, enabling rapid insertions and membership checks while managing the unique positions the guard visits. Additionally, for part two, I implemented parallel processing with Rayon to validate potential obstacle placements, helping determine the positions leading to a loop efficiently.
</details>
            
<!---DAY6_END-->

<!---DAY7_BEGIN-->

### [⭐️⭐️ Day 7 - Bridge Repair](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day07/mod.rs)
            
<details>
<summary>See explanation</summary>

I leveraged combinatorial algorithms to systematically evaluate all possible configurations of operators ('+' and '*') between operands in each equation using a parallelized approach for efficiency. By utilizing `itertools` for generating combinations via `multi_cartesian_product`, I ensured that every possible operator arrangement was evaluated in a left-to-right manner, as prescribed by the problem. The results of these computations were accumulated only if they equated to the specified target value. The main data structure employed was a simple tuple `(i64, Vec<i64>)` to represent each equation, optimizing both clarity and access during calculations. This method not only enhances performance through parallel processing with `rayon` but also allows for direct and expressive manipulation of the operator configurations across potentially large datasets.
</details>
            
<!---DAY7_END-->

<!---DAY8_BEGIN-->

### [⭐️⭐️ Day 8 - Resonant Collinearity](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day08/mod.rs)
            
<details>
<summary>See explanation</summary>

To tackle the problem of finding antinodes created by cell towers, I utilized a combination of combinatorial iteration and vector arithmetic. I first defined a `CellTower` struct to hold the position and frequency of each antenna. Using the `combinations` method from the `itertools` crate, I iterated through all possible pairs of antennas and filtered them by frequency. For valid pairs, I computed the direction between the two antennas and employed a loop to generate potential antinodes along that direction until moving out of bounds. This approach ensured I captured all valid antinode positions efficiently, and by leveraging a vector for accumulation, I could effectively collect and count unique antinode locations using a helper function that utilized Rust's iterator methods.
</details>
            
<!---DAY8_END-->

<!---DAY9_BEGIN-->

### [⭐️⭐️ Day 9 - Disk Fragmenter](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day09/mod.rs)
            
<details>
<summary>See explanation</summary>

My approach to solving the puzzle involves the design of a `Disk` struct that encapsulates the disk representation using a vector of `DiskBlock` enums to distinguish between file blocks and free space. I implemented two defragmentation methods: `defrag_simple`, which efficiently moves files to the left using a two-pointer technique to swap blocks until all files are contiguous, and `defrag_files`, which identifies files and free regions to intelligently relocate files based on their sizes and starting positions. The `checksum` method computes the final checksum by iterating through the blocks, calculating the product of each block's position and its corresponding file ID. Additional helper methods organize the logic for identifying file and free regions, ensuring the solution remains modular and maintainable while effectively managing the underlying complexities of the data structure.
</details>
            
<!---DAY9_END-->

<!---DAY10_BEGIN-->

### [⭐️⭐️ Day 10 - Hoof It](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day10/mod.rs)
            
<details>
<summary>See explanation</summary>

I employed a recursive depth-first search approach to explore the hiking trails on the topographic map, leveraging a `HashSet` to keep track of unique reachable positions from each trailhead. The `find_trails` function identifies paths leading to height `9` by recursively checking neighbors and ensuring that the height increases by exactly `1` with each step. I created two functions: `trailhead_score`, which counts the distinct reachable `9` positions for a trailhead, and `trailhead_rating`, which also utilizes recursion to achieve this task. The core logic is efficiently implemented using a grid representation, with a final aggregation of scores performed in the `map_score` function, iterating through the grid and applying the scoring functions to each trailhead.
</details>
            
<!---DAY10_END-->

<!---DAY11_BEGIN-->

### [⭐️⭐️ Day 11 - Plutonian Pebbles](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day11/mod.rs)
            
<details>
<summary>See explanation</summary>

I utilized a combination of recursive functions and memoization via a `HashMap` to efficiently handle the evolution of the stones over multiple blinks. The `apply_rules` function applies the specific transformation rules to a single stone: replacing it with either a new value, splitting it into two, or multiplying it by 2024 based on its current state. By recursively processing the list of stones, I maintained a count of the resulting stones after each blink while leveraging the `HashMap` to store previously computed results for pairs of a stone and the number of blinks left, thereby avoiding redundant calculations and significantly improving performance. This approach ensured that the transformation met the puzzle's requirements while managing the potentially large growth in the number of stones.
</details>
            
<!---DAY11_END-->

<!---DAY12_BEGIN-->

### [⭐️⭐️ Day 12 - Garden Groups](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day12/mod.rs)
            
<details>
<summary>See explanation</summary>

I utilized a depth-first search (DFS) algorithm to identify distinct regions of garden plots in the grid, tracking visited plots to avoid redundancy. For each unvisited plot, I counted the area while collecting edge information, leveraging a `HashSet` to maintain unique edges efficiently. This allowed me to compute both the area and perimeter based on the identified edges. To calculate the price of the fencing for each region, I implemented two methods: one that multiplies area by the number of edges and another that adjusts for shared edges at right angles, improving accuracy by using the geometrical properties of the regions. The separation of edge handling into two distinct functions allows for flexible calculations based on differing perimeter interpretations.
</details>
            
<!---DAY12_END-->

<!---DAY13_BEGIN-->

### [⭐️⭐️ Day 13 - Claw Contraption](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day13/mod.rs)
            
<details>
<summary>See explanation</summary>

My approach involves modeling each claw machine's button configurations and prize locations as a `ClawMachine` struct, which captures the movement values for buttons A and B, as well as the target prize coordinates. I implement a method to solve a system of linear equations derived from the machine's movements relative to the prize location, effectively determining the number of times each button needs to be pressed to attain the prize coordinates. This is computed by equating the coefficients of the movements to the prize coordinates and checking for valid, non-negative solutions. I leverage a simple cost calculation method that retrieves these solutions and sums the associated token costs, allowing me to comprehensively aggregate the total cost across all machines to determine the minimum tokens required to win the maximum number of prizes.
</details>
            
<!---DAY13_END-->

<!---DAY14_BEGIN-->

### [⭐️⭐️ Day 14 - Restroom Redoubt](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day14/mod.rs)
            
<details>
<summary>See explanation</summary>

I structured the solution by defining two primary structs: `Robot`, which encapsulates each robot's position and velocity, and `Map`, which manages a collection of robots and their movements within a defined grid space. The robots' updates are performed in the `step` method, leveraging modular arithmetic to accommodate the teleportation behavior when they exceed the grid boundaries. The `safety_factor` is calculated by counting the robots in each quadrant of the grid, using a straightforward comparison of positions against grid midpoints. To summarize the robot positions visually, I implemented an `as_grid` method, converting their coordinates into a 2D character array. Additionally, a scoring heuristic for regularity was included in the `Grid` struct to evaluate the layout, enhancing potential future use cases. This combination of structs and methods achieved both simulation and assessment of robot distribution effectively.
</details>
            
<!---DAY14_END-->

<!---DAY15_BEGIN-->

### [⭐️⭐️ Day 15 - Warehouse Woes](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day15/mod.rs)
            
<details>
<summary>See explanation</summary>

I implemented a simulation of the robot's movements within a grid representing the warehouse using a custom `Warehouse` struct, which encapsulates a `Grid` of `WarehouseCell` enums to represent different entities like walls, boxes, and the robot. To handle box pushing mechanics efficiently, I utilized a recursive approach in `get_pushable_boxes` that collects all adjacent boxes in the push direction, allowing for a systematic update of their positions when the robot attempts to move. Additionally, I created methods for directional movement that factor in the robot's ability to push boxes while respecting the boundaries and interactions with walls, ensuring the integrity of the simulation remains intact. Finally, after executing the movements, I calculated the total GPS coordinates of the remaining boxes to produce the required outcome.
</details>
            
<!---DAY15_END-->

<!---DAY16_BEGIN-->

### [👨‍💻👨‍💻 Day 16 - Reindeer Maze](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day16/mod.rs)
            
<details>
<summary>See explanation</summary>


</details>
            
<!---DAY16_END-->

<!---DAY17_BEGIN-->

### [👨‍💻👨‍💻 Day 17 - Chronospatial Computer](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day17/mod.rs)
            
<details>
<summary>See explanation</summary>


</details>
            
<!---DAY17_END-->

<!---DAY18_BEGIN-->

### [👨‍💻👨‍💻 Day 18 - RAM Run](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day18/mod.rs)
            
<details>
<summary>See explanation</summary>


</details>
            
<!---DAY18_END-->

<!---DAY19_BEGIN-->

### [⭐️⭐️ Day 19 - Linen Layout](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day19/mod.rs)
            
<details>
<summary>See explanation</summary>

I employed a dynamic programming approach to solve this puzzle, using a vector `memo` to store counts of ways to construct substrings of the design from the available towel patterns. I initialized the first element of `memo` to 1, representing the base case of matching an empty substring, and iterated over each possible substring of the desired design. For each substring, I checked if it was present in a `HashSet` of the available patterns, updating the `memo` counts based on previous calculations, thus enabling O(n^2) time complexity while ensuring efficient membership checks with the hash set. This structured approach allowed me to count how many valid combinations of towel patterns could recreate each design effectively.
</details>
            
<!---DAY19_END-->

<!---DAY20_BEGIN-->

### [👨‍💻👨‍💻 Day 20 - Race Condition](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day20/mod.rs)
            
<details>
<summary>See explanation</summary>


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
