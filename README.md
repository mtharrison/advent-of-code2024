<h1 align="center">🦀 Advent of Code 2024 🎄</h1>

<div align="center">

<a href="">![build](https://github.com/mtharrison/advent-of-code2024/actions/workflows/rust.yml/badge.svg)</a>
<a href="">[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)</a>

</div>

My solutions to [AOC 2024](https://adventofcode.com/2024/about) in [The Rust Programming Language](https://www.rust-lang.org/).

---

> _The following info is [generated automatically](https://github.com/mtharrison/advent-of-code2024/tree/main/bin/readme)_

<!---DAY1_BEGIN-->

### [⭐️⭐️ Day 1 - Historian Hysteria](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day01/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
To solve the puzzle, I employed the use of two <code>Vec<i32></code> to efficiently store and manipulate the location IDs from both groups of historians. First, I parsed the input to separate the left-hand side (lhs) and right-hand side (rhs) lists, which I then sorted using the <code>sort()</code> method to facilitate pairing. For calculating the total distance, I iterated through the sorted vectors, summing the absolute differences of the paired elements to get the final distance. Additionally, I created a <code>filter()</code> operation on the rhs vector in the <code>get_similarity</code> function to count occurrences of each element from lhs, allowing me to compute a similarity score based on the frequency of the numbers. This approach leverages sorting and simple iterations, ensuring efficient calculations for the distances and similarities between the two lists.
</details>
            
<!---DAY1_END-->

<!---DAY2_BEGIN-->

### [⭐️⭐️ Day 2 - Red-Nosed Reports](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day02/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
In my solution, I utilized a recursive approach to validate the safety of the reports based on the specified conditions. I designed the function <code>is_safe</code> to check if the levels in a report are either entirely increasing or decreasing while ensuring that adjacent differences remain within the tolerance range of 1 to 3. If a violation occurs and tolerance is allowed, I tested alternative lists by removing elements to see if the remaining numbers could form a valid sequence, which maintains the safety criteria through the recursive calls. I used a straightforward iterative loop in the <code>count_safe</code> function to count the number of reports that are safe, effectively managing the complexity of the checks. Overall, I leveraged the cloning and reversal of input vectors strategically to handle the two directional checks for safety.
</details>
            
<!---DAY2_END-->

<!---DAY3_BEGIN-->

### [⭐️⭐️ Day 3 - Mull It Over](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day03/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I used a <code>Regex</code> to identify valid <code>mul</code> instructions within the corrupted input, which allows me to extract the relevant numerical values while ignoring any invalid characters. Each valid instruction is transformed into an <code>Instruction</code> enum, enabling clear differentiation between multiplication and control commands. I implemented a <code>Machine</code> struct that maintains an accumulator for results and a state indicating whether execution of multiplication instructions is enabled based on control commands. The <code>run_program</code> method iterates through the parsed instructions and executes them, allowing for a straightforward accumulation of results that can be used to derive the final answer.
</details>
            
<!---DAY3_END-->

<!---DAY4_BEGIN-->

### [⭐️⭐️ Day 4 - Ceres Search](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day04/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I used a custom <code>Grid</code> data structure to represent the letter grid, which allows for easy access to rows, columns, and diagonals. To find occurrences of the target words "XMAS" and "SAMX", I created an iterator over the flattened search results from these dimensions. I then collected each line as a <code>String</code> and used the <code>matches</code> method to count occurrences efficiently. For the second part of the puzzle, I implemented a function <code>is_xmas_grid</code> that checks 3x3 squares to see if they contain specific arrangements of the letters, further aggregating results across multiple test cases.
</details>
            
<!---DAY4_END-->

<!---DAY5_BEGIN-->

### [⭐️⭐️ Day 5 - Print Queue](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day05/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I employed a tuple-based representation using the type <code>Edge</code> to encapsulate the ordering rules and a vector <code>Path</code> to represent the sequences of pages. To determine if a path adheres to the rules, I created the <code>valid_path</code> function, which checks adjacent page pairs against the ordering rules. When a path is invalid, I extract only the relevant edges using <code>pick_edges</code>, filtering rules that pertain to the current path. For invalid paths, I utilized a directed acyclic graph (DAG) with the <code>Graph</code> structure to perform a topological sort, allowing me to find the correct order for the pages. Finally, I computed the middle page number from valid updates and aggregated the results for the output.
</details>
            
<!---DAY5_END-->

<!---DAY6_BEGIN-->

### [⭐️⭐️ Day 6 - Guard Gallivant](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day06/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I designed the solution by modeling the world as a grid using a <code>Vec<Vec<Cell>></code>, where each cell can represent either a guard, obstacle, visited position, or vacant space. The guard's state, including its position and direction, is encapsulated in a <code>GuardState</code> struct, while movement and interaction logic are handled in the <code>World</code> struct through the <code>step</code> method, which processes the guard's movement and updates the grid accordingly. To track positions visited by the guard, I employed a <code>HashSet</code> to store unique coordinates, ensuring efficient O(1) lookup and insertion. The traversal logic makes use of matching and condition checking to determine movements effectively, taking care to handle direction changes upon encountering obstacles. This structure not only maintains clean code but also allows for a straightforward implementation of the simulation.
</details>
            
<!---DAY6_END-->

<!---DAY7_BEGIN-->

### [⭐️⭐️ Day 7 - Bridge Repair](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day07/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I used a combination of parallel processing and combinatorial generation to evaluate each equation against possible operator placements. By leveraging the <code>rayon</code> crate for parallel iteration, I ensured that the computation could handle a large input efficiently. For each equation, I employed <code>multi_cartesian_product</code> from the <code>itertools</code> library to generate all possible combinations of operators ('+', '*', and '|') placed between operands. The evaluation of these combinations occurs iteratively, where I maintained an accumulator to compute the result based on the selected operators. If the computed result matches the expected value, I added it to a running total. This approach allows me to efficiently check multiple operators across numerous operand configurations while ensuring correctness through straightforward accumulation and conditional checks.
</details>
            
<!---DAY7_END-->

<!---DAY8_BEGIN-->

### [⭐️⭐️ Day 8 - Resonant Collinearity](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day08/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a combination of the <code>Vec</code> and <code>Vec2d</code> data structures to effectively manage the two-dimensional representation of the map and the antennas. To identify antinodes, I employed the <code>combinations</code> method from the <code>itertools</code> library to generate pairs of antennas with matching frequencies. In the <code>find_antinodes_pt1</code> and <code>find_antinodes_pt2</code> functions, I calculated potential antinode positions based on the distance between antennas while ensuring they remained within map bounds using the <code>out_of_bounds</code> check. By using <code>flat_map</code> and <code>filter</code>, I efficiently collected unique antinode coordinates and calculated their count using <code>unique</code>, which streamlined the process of obtaining the final result.
</details>
            
<!---DAY8_END-->

<!---DAY9_BEGIN-->

### [⭐️⭐️ Day 9 - Disk Fragmenter](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day09/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a combination of custom data structures to represent the disk state efficiently, specifically using a <code>Vec</code> to store <code>DiskBlock</code> enums for files and free spaces. The main defragmentation is accomplished through the <code>defrag_simple</code> method, which iteratively swaps blocks while maintaining the indices of the first free space and the last block. I also implemented a more advanced <code>defrag_files</code> that seeks optimal free regions for file moves, leveraging vectors to temporarily store file and free region data, ensuring files are properly reorganized while keeping track of their IDs. Finally, I calculated the checksum by iterating through the blocks and multiplying their positions by their corresponding IDs, accumulating the total in an <code>i64</code> for accurate summation.
</details>
            
<!---DAY9_END-->

<!---DAY10_BEGIN-->

### [⭐️⭐️ Day 10 - Hoof It](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day10/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a recursive depth-first search approach to explore the hiking trails in the topographic map. The core of the solution involves two functions, <code>find_trails</code> and <code>distinct_trails</code>, which both traverse the grid starting from a trailhead at height <code>0</code>. I leveraged a <code>HashSet</code> to keep track of reachable positions during the search, ensuring that duplicate positions were not counted when calculating the score for each trailhead. The <code>map_score</code> function iteratively evaluates each position in the grid, applying either <code>trailhead_score</code> or <code>trailhead_rating</code> to determine the score based on reachable heights, while accumulating the results in a vector for efficient summation. This design facilitates a clear separation of concerns, making it easy to adapt the scoring function as needed.
</details>
            
<!---DAY10_END-->

<!---DAY11_BEGIN-->

### [⭐️⭐️ Day 11 - Plutonian Pebbles](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day11/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I structured my solution using a series of functions to apply the transformation rules on the integer representations of the stones. I utilized three distinct functions for the rules: <code>rule1</code> for the 0-to-1 transformation, <code>rule2</code> for splitting even-digit numbers into two stones, and <code>rule3</code> for multiplying by 2024 for other cases. The main transformation is executed in the <code>apply_rules</code> function, which checks the applicable rules in order. To efficiently handle repeated calculations, I implemented a recursive function <code>blink_recursive_count</code> with memoization using a <code>HashMap</code> to store previously computed results based on current stone values and remaining blinks, thereby significantly reducing redundant computations over multiple iterations. Overall, this design optimizes the processing of transformations across potentially exponential growth in stone counts resulting from successive blinks.
</details>
            
<!---DAY11_END-->

<!---DAY12_BEGIN-->

### [⭐️⭐️ Day 12 - Garden Groups](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day12/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I used a depth-first search algorithm to identify and traverse regions of garden plots within a grid, tracking visited plots using a <code>HashSet</code> to avoid re-exploration. Each region's area is calculated by counting the number of plots visited, while the perimeter is computed by collecting the edges of the region, using a <code>HashSet</code> to ensure no duplicates. For perimeter calculations, I leveraged the relationship between edges and vertices to determine which edges contribute to the perimeter based on adjacency rules. I then calculated the total price of fence for all regions, both by multiplying area and edge count, and by using a refined method based on edge orientation, ensuring efficient data handling and accurate calculations throughout.
</details>
            
<!---DAY12_END-->

<!---DAY13_BEGIN-->

### [⭐️⭐️ Day 13 - Claw Contraption](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day13/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I designed a <code>ClawMachine</code> struct to encapsulate the button behaviors and prize locations, which allows for easily managing the parameters necessary for each machine. To solve the problem, I employed a method for calculating the minimum token cost to reach the prize using linear equations, manipulating coefficients derived from the claw's movements and button costs. The <code>solve</code> function computes potential button presses by solving for the number of times each button must be pressed, and the <code>cost</code> function aggregates these results, ensuring that only valid combinations of button presses are considered. By iterating through all machines and summing their costs, I effectively obtain the total minimum token expenditure required to claim all prizes.
</details>
            
<!---DAY13_END-->

<!---DAY14_BEGIN-->

### [⭐️⭐️ Day 14 - Restroom Redoubt](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day14/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I used a <code>Vec</code> to store the collection of <code>Robot</code> instances, each containing their position and velocity, represented as <code>Vec2d</code> structs. The <code>Map</code> struct manages robot movements and handles the wrapping behavior at the edges of the defined grid by utilizing modulo arithmetic during position updates in the <code>step</code> function. I implemented the robot simulation by iterating through a specified number of steps in the <code>step_n_times</code> method, while the <code>count_quadrants</code> method calculates the number of robots in each quadrant and returns the counts as an array. Finally, I compute the safety factor by multiplying the counts of robots in each quadrant using the <code>product</code> method on the counts array, ensuring efficient and clear calculations to address the problem requirements.
</details>
            
<!---DAY14_END-->

<!---DAY15_BEGIN-->

### [⭐️⭐️ Day 15 - Warehouse Woes](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day15/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a custom <code>Grid</code> structure to represent the warehouse, where each cell can be a wall, vacant space, box, or robot, thus allowing for effective manipulation during the simulation of movements. The robot's movements are processed sequentially, with the algorithm checking surrounding cells to see if it can move or push boxes, relying on recursion to gather pushable boxes in the given direction. Additionally, to handle the scenario of boxes being split into halves during movements, I implemented an enum <code>Box</code> with variants for single boxes and their halves, ensuring accurate tracking of their states. The GPS score is calculated after all movements by summing up the calculated positions of the boxes based on their distance from the warehouse edges. The structure and methods of <code>Warehouse</code> thus efficiently manage the simulation and the subsequent scoring process.
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
I utilized dynamic programming to solve the problem of determining how many designs can be created from the available towel patterns. I maintained a memoization vector, <code>memo</code>, where <code>memo[i]</code> represents the number of ways to form the substring of the design from the start up to index <code>i</code>. For each character index <code>i</code> in the design, I examined all possible substrings ending at that index, checking if they exist in a <code>HashSet</code> of available patterns. If a valid substring is found, I added the number of ways to form the preceding substring to <code>memo[i]</code>. This approach ensures that I efficiently compute the number of ways to achieve each design by reusing previously computed results, leading to a final count that is maintained through the memoization technique.
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
