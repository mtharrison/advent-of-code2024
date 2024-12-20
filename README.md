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
I utilized two primary functions to solve the problem: <code>get_distance</code> and <code>get_similarity</code>. For calculating the total distance between two lists, I first sorted both <code>lhs</code> and <code>rhs</code> using the <code>sort</code> method, which allows me to efficiently pair the smallest elements from each list. I then iterated over the sorted vectors, computing the absolute difference for each corresponding pair and accumulating this into a <code>distance</code> variable. To determine the similarity, I employed an iteration over the <code>lhs</code> list, using a filter to count occurrences of each element in <code>rhs</code>, summing the results multiplied by the element's value into a <code>similarity</code> variable. This approach leverages vectors and sorting to maintain clarity and efficiency in the calculations.
</details>
            
<!---DAY1_END-->

<!---DAY2_BEGIN-->

### [⭐️⭐️ Day 2 - Red-Nosed Reports](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day02/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I used a combination of recursive function calls and vector manipulation to determine if each report is safe based on the specified criteria. In the <code>is_safe</code> function, I iterated through the report and calculated the difference between adjacent levels, checking if any difference fell outside the defined range. If a difference was found that exceeded the tolerance, I cloned the report, removing the problematic levels and recursively checking the altered reports to see if they could still be classified as safe without those elements. The <code>check_safe</code> function handled both increasing and decreasing checks by reversing the report and passing it back to <code>is_safe</code>. Finally, <code>count_safe</code> iterated through all reports and incremented the count based on the results of the checks, allowing for a straightforward determination of how many reports were ultimately classified as safe.
</details>
            
<!---DAY2_END-->

<!---DAY3_BEGIN-->

### [⭐️⭐️ Day 3 - Mull It Over](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day03/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a finite state machine approach by defining an `Instruction` enum with variants for multiplication, enabling, and disabling operations. The `Machine` struct maintains an accumulator for storing results and flags to control execution. In the `scan_program` function, I employed a regular expression to identify valid `mul` instructions and selectively parse them while ignoring invalid ones. This approach allowed for efficient accumulation of results from the `mul` operations as they are executed in sequence, ensuring performance remains optimal even with corrupted inputs. The machine can also accommodate different behaviors based on flags, enhancing its versatility for differing problem requirements.
</details>
            
<!---DAY3_END-->

<!---DAY4_BEGIN-->

### [⭐️⭐️ Day 4 - Ceres Search](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day04/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I approached the puzzle by utilizing a combination of structured grid data and string manipulation to efficiently search for occurrences of the word "XMAS". I implemented a <code>Grid</code> struct to represent the 2D letter array, allowing easy access to rows, columns, and diagonals. In the main search logic, I collected all possible search patterns into a vector and flattened it for iteration, enabling quick string comparisons using the <code>matches</code> method for both "XMAS" and its reverse "SAMX". This way, I ensured that I accounted for all orientations and placements of the target word. Additionally, I implemented a helper function, <code>is_xmas_grid</code>, to check 3x3 square sections of the grid for the presence of "XMAS", further streamlining the counting process through structured searching.
</details>
            
<!---DAY4_END-->

<!---DAY5_BEGIN-->

### [⭐️⭐️ Day 5 - Print Queue](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day05/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
To solve the puzzle, I utilized a vector to store the edges of page ordering rules, along with a vector of vectors to store the paths of updates. The function `valid_path` checks if a given path adheres to these rules by verifying the order of consecutive pages. For cases where the path is not valid, I extracted the relevant edges using the `pick_edges` function, which filters to only those rules applicable to the current path. I then constructed a directed acyclic graph (DAG) from these edges and performed a topological sort to determine a valid ordering of the pages. This allowed me to find the middle page number for each correctly ordered update and accumulate their sum for the final result. The overall approach effectively combines graph theory concepts with path validation to meet the problem's requirements.
</details>
            
<!---DAY5_END-->

<!---DAY6_BEGIN-->

### [⭐️⭐️ Day 6 - Guard Gallivant](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day06/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a combination of enums and a 2D vector to represent the state of the world and the guard's behavior. Specifically, I defined a <code>World</code> struct containing a grid of <code>Cell</code> enums (to denote the guard, vacant spaces, obstacles, and visited positions) and a <code>GuardState</code> struct to manage the guard's position and direction. The guard's movement is handled in the <code>step</code> method, where I apply the patrol logic—moving forward or turning right based on the presence of obstacles. For tracking visited positions, I employed a <code>HashSet</code> to store unique coordinates, efficiently preventing duplicate visits and allowing for quick look-up. The logic elegantly handles direction changes and collision with obstacles while maintaining clarity in representing the guard's movement across the grid.
</details>
            
<!---DAY6_END-->

<!---DAY7_BEGIN-->

### [⭐️⭐️ Day 7 - Bridge Repair](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day07/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I leveraged parallel computation using the <code>rayon</code> library to efficiently evaluate all possible configurations of operators for multiple equations simultaneously. I stored the test results and operands in a tuple type named <code>Equation</code>, which allowed for simple access and iteration. By using <code>multi_cartesian_product</code> from the <code>itertools</code> crate, I generated all combinations of the defined operators between operands, facilitating the evaluation of each configuration left-to-right. For each equation, I maintained an accumulator to apply the operators sequentially and compared the resulting value against the expected test value, summing the valid results as I processed each equation.
</details>
            
<!---DAY7_END-->

<!---DAY8_BEGIN-->

### [⭐️⭐️ Day 8 - Resonant Collinearity](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day08/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I leveraged the <code>Vec</code> and <code>HashSet</code> data structures to efficiently compute and store the unique antinode positions created by pairs of antennas with the same frequency. In the <code>find_antinodes_pt1</code> function, I used the <code>itertools::combinations</code> method to generate pairs of antennas and computed the positions of potential antinodes based on their distances, ensuring that I only considered those within bounds using the <code>out_of_bounds</code> function. For the second part in <code>find_antinodes_pt2</code>, I extended the calculation to consider multiple antennas by iteratively adding positions until I went out of bounds, while still filtering unique antinode positions through the count of a <code>HashSet</code>. This approach allowed me to consolidate position checking and uniqueness verification in a cohesive manner, efficiently obtaining the required result.
</details>
            
<!---DAY8_END-->

<!---DAY9_BEGIN-->

### [⭐️⭐️ Day 9 - Disk Fragmenter](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day09/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I structured my solution using a custom `Disk` type to represent the blocks of files and free space, implemented with a `Vec<DiskBlock>` to efficiently manage the dynamic nature of block rearrangements. I defined two primary methods for defragmentation: `defrag_simple` for a straightforward approach that swaps blocks, and `defrag_files`, which intelligently relocates files based on free regions, utilizing helper functions like `swap_multiple`, `free_regions`, and `files` to manage internal state and querying. The checksum calculation is performed by iterating through the blocks and aggregating the product of each block's position and its file ID, filtering out free spaces to ensure only relevant contributions are summed. This design leverages Rust’s type system to maintain clarity and performance throughout the operation processes.
</details>
            
<!---DAY9_END-->

<!---DAY10_BEGIN-->

### [⭐️⭐️ Day 10 - Hoof It](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day10/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I implemented a depth-first search approach to traverse the grid by defining two primary functions: <code>find_trails</code> and <code>distinct_trails</code>. These functions recursively explore valid hiking trails starting from trailheads (positions with height <code>0</code>), incrementing the height on each step until reaching <code>9</code>. I utilized a <code>HashSet</code> to store unique reachable positions for the <code>find_trails</code> function, ensuring that I count each <code>9</code> once. To calculate the scores, I iterated through all grid positions in the <code>map_score</code> function, invoking the respective score function based on the criteria laid out in the puzzle, effectively summing the scores for each trailhead. This systematic methodology allows for efficient computation of reachable heights from all trailheads in the grid.
</details>
            
<!---DAY10_END-->

<!---DAY11_BEGIN-->

### [⭐️⭐️ Day 11 - Plutonian Pebbles](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day11/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a recursive approach combined with memoization to solve the problem efficiently. Each stone's transformation is handled by three rules implemented as separate functions: <code>rule1</code>, <code>rule2</code>, and <code>rule3</code>, which process the input stone based on its current value. The transformations are applied iteratively through the <code>blink_recursive_count</code> function, which leverages a <code>HashMap</code> to store previously calculated results for specific stone states and blink counts, minimizing redundant calculations. This allows me to efficiently compute the final stone count after a specified number of blinks while ensuring that the order of transformations is preserved.
</details>
            
<!---DAY11_END-->

<!---DAY12_BEGIN-->

### [⭐️⭐️ Day 12 - Garden Groups](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day12/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
To solve the problem, I employed a depth-first search approach to identify and collect regions of connected garden plots while tracking the number of plots and their contributing edges. I utilized a <code>HashSet</code> to efficiently manage visited coordinates and another <code>HashSet</code> to store edges for each identified region to avoid duplicates. The edges are categorized by direction, allowing for easy calculations of both area and perimeter. In the final price calculations, I created two functions: one that calculates the price based on the edges count and another utilizing the sides of the regions, leveraging the right-angle insight to evaluate how edges contribute to the total perimeter. This design ensured both clarity and performance in processing the grid structure.
</details>
            
<!---DAY12_END-->

<!---DAY13_BEGIN-->

### [⭐️⭐️ Day 13 - Claw Contraption](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day13/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I used a struct <code>ClawMachine</code> to encapsulate the behavior of each claw machine, storing the movement values for buttons A and B as well as the prize coordinates. The <code>solve</code> method implements the solution to the problem by solving a set of linear equations that determine the number of presses for each button required to align the claw with the prize. This is achieved by calculating coefficients and ensuring the results are valid integers, particularly checking for non-zero denominators and divisibility. The <code>cost</code> method computes the total token cost based on the number of button presses calculated. Lastly, I leveraged the <code>From</code> trait to parse the input strings into <code>ClawMachine</code> instances, and collected all machines into a vector for processing, which simplifies the handling of multiple machines.
</details>
            
<!---DAY13_END-->

<!---DAY14_BEGIN-->

### [⭐️⭐️ Day 14 - Restroom Redoubt](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day14/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
To solve the problem, I designed a structure to represent the robots and their movements on a grid, using a <code>Vec</code> of <code>Robot</code> instances for efficient access and modification. The <code>Map</code> struct holds the robots and the grid dimensions, allowing for a method <code>step</code> that updates each robot's position based on its velocity while implementing the teleportation behavior using modular arithmetic to wrap around the edges. To count the robots in each quadrant after a specified number of steps, I utilized an array to keep track of the counts and a matching strategy in <code>count_quadrants</code> based on their positional coordinates. Finally, the <code>safety_factor</code> is calculated by multiplying the counts of robots in each quadrant, allowing for efficient computation and modularity in the design.
</details>
            
<!---DAY14_END-->

<!---DAY15_BEGIN-->

### [⭐️⭐️ Day 15 - Warehouse Woes](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day15/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a grid-based approach to model the warehouse, implementing a custom <code>Grid</code> data structure to manage the state of each cell, representing walls, vacant spaces, boxes, and the robot. Each move of the robot was processed sequentially, considering the potential to push boxes in the designated direction. For box movement, I recursively gathered pushable boxes based on their positions and the pushing direction using a depth-first search approach. I implemented separate methods to handle upwards/downwards and left/right movements of boxes, ensuring proper updates to the grid. The GPS score for boxes was calculated by iterating over the grid after all moves, summing the computed coordinates of each box based on its position, thus efficiently tracking changes throughout the robot's movements.
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
I used a dynamic programming approach to efficiently determine the number of ways to construct each design using the available towel patterns. To do this, I created a vector <code>memo</code> to store the number of ways to construct each substring of the design, initializing <code>memo[0]</code> to 1 for the base case. I utilized a <code>HashSet</code> to store the available patterns for O(1) lookup during the substring evaluation. For each position in the design, I iterated backwards to check each possible ending substring and updated the <code>memo</code> vector accordingly based on whether the substring matched any pattern in the <code>HashSet</code>. This allowed me to efficiently count possible ways for each design in linear time relative to its length, resulting in an overall efficient solution.
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
