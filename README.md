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
To solve the puzzle, I utilized two vectors to store the location IDs from the left and right lists, enabling efficient indexing for comparison. After parsing the input, I sorted both vectors to ensure that corresponding elements could be paired by their order. The `get_distance` function calculates the total distance between these pairs by iterating through the sorted vectors and summing the absolute differences of their corresponding elements. For the additional task of calculating similarity, I employed an iterator and a filter to count the occurrences of each left-side ID in the right-side vector while accumulating a weighted total based on the ID's value. This approach leverages Rust’s strong type system and efficient collections, making both operations straightforward and performant.
</details>
            
<!---DAY1_END-->

<!---DAY2_BEGIN-->

### [⭐️⭐️ Day 2 - Red-Nosed Reports](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day02/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I approached the problem by defining a recursive function, `is_safe`, that checks the safety of each report based on the specified conditions. I utilized a vector of vectors to store the parsed reports and iterated through them to analyze adjacent differences. If a difference violated the tolerance, I cloned the report to create modified versions (with one element removed) and recursively checked their safety. This allows me to efficiently determine if a report is still safe even when one number is discarded, thus handling both scenarios of tolerance. The `check_safe` function checks both the original and the reversed report to account for increasing or decreasing sequences. Finally, I aggregated the results with the `count_safe` function, maintaining a count of all safe reports.
</details>
            
<!---DAY2_END-->

<!---DAY3_BEGIN-->

### [⭐️⭐️ Day 3 - Mull It Over](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day03/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a state machine design, implementing the `Machine` struct to process a sequence of instructions encapsulated in an `Instruction` enum. This approach allows for encapsulation of state, where the `accumulator` keeps track of the ongoing sum of multiplication results derived from `mul` instructions. I employed a regex pattern to identify valid `mul` instructions while ignoring corrupted segments, enabling efficient parsing. The execution logic is straightforward, enabling or disabling the handling of valid instructions based on the machine's state, thus allowing for dynamic control over how instructions are processed. This structure not only keeps the processing logic clear and flexible but also facilitates easy testing and verification of both parts of the puzzle with distinct configurations.
</details>
            
<!---DAY3_END-->

<!---DAY4_BEGIN-->

### [⭐️⭐️ Day 4 - Ceres Search](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day04/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
In my solution, I utilized a custom `Grid` data structure to represent the 2D character grid effectively, allowing me to easily access both rows and columns, as well as the diagonals of the grid. For searching the term "XMAS" and its reverse "SAMX", I concatenated the characters from each possible search direction into strings, leveraging Rust's powerful iterator functionalities to streamline the process. This approach allowed me to count occurrences of the target strings efficiently by using the `matches` method on these concatenated strings. Additionally, for part two, I created a function to examine all 3x3 sections of the grid, applying a helper function to check for the presence of specific sequences, thereby encapsulating the logic cleanly and maintaining clarity in my implementation.
</details>
            
<!---DAY4_END-->

<!---DAY5_BEGIN-->

### [⭐️⭐️ Day 5 - Print Queue](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day05/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
To solve the puzzle, I implemented a method to check if the pages in each update are printed in accordance with the rules. I used tuples to represent the edges of the dependency rules as pairs of page numbers, stored in a vector. For checking the validity of the paths, I iterated through each page in the update and validated the sequential order against the rules. If a path was not valid, I gathered the relevant rules for that specific path to construct a directed acyclic graph (DAG) and performed a topological sort on it, using a separate `Graph` struct. Finally, I summed the middle page numbers of all valid paths to obtain the desired result.
</details>
            
<!---DAY5_END-->

<!---DAY6_BEGIN-->

### [⭐️⭐️ Day 6 - Guard Gallivant](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day06/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
In my solution, I utilized an object-oriented approach with structs representing key entities such as the `Guard`, `Cell`, and the `World`. For simulating the guard’s movement, I implemented a state machine-like behavior within the `World` struct where the guard's position and direction could be updated based on obstacles encountered. I used a `HashSet` to efficiently track both visited positions and the guard's states to identify any loops. This directed approach ensures that I handle the finite grid of the world effectively, managing the transitions between states while leveraging Rust's ownership model and strong typing to maintain clarity and correctness. Additionally, I leveraged Rayon for parallel processing when checking the impact of potential new obstacles, thus maximizing performance in scenarios where multiple evaluations are required.
</details>
            
<!---DAY6_END-->

<!---DAY7_BEGIN-->

### [⭐️⭐️ Day 7 - Bridge Repair](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day07/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I approached the problem by first using a tuple data structure to represent each equation as a pair of the expected result and a vector of operands. The core computation is handled in the `sum_valid` function, which employs parallel iteration over the input equations to maximize performance, especially given the potentially large combinatorial nature of the operator arrangements. I utilized the `itertools` library to generate all possible operator combinations with a multi-cartesian product and evaluated each combination left-to-right, checking if it matches the expected result. A summation collects the valid results while ensuring efficient handling of the data using Rust's ownership model. This way, I efficiently determined which equations could potentially be true while maintaining clarity in operations.
</details>
            
<!---DAY7_END-->

<!---DAY8_BEGIN-->

### [⭐️⭐️ Day 8 - Resonant Collinearity](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day08/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a combination of Rust's iterators and the itertools library to efficiently compute the positions of antinodes between pairs of antennas. The `find_antinodes_pt1` function employs combinations to identify pairs of antennas with matching frequencies, calculating potential antinode positions based on distance, while ensuring these positions remain within bounds using a dedicated `out_of_bounds` function. For part two, the algorithm iterates through valid sequences based on antenna positions, again leveraging iterators to append multiple usable positions in a straightforward manner. This dual approach allows for flexibility in collecting unique antinode locations while maintaining performance with set operations to ensure uniqueness.
</details>
            
<!---DAY8_END-->

<!---DAY9_BEGIN-->

### [⭐️⭐️ Day 9 - Disk Fragmenter](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day09/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I implemented a solution where I represented the disk using a `Vec<DiskBlock>`, allowing for efficient manipulation of blocks that can either be free space or contain files identified by their unique IDs. To handle the defragmentation process, I created two methods: `defrag_simple` which swaps the blocks to move files to the leftmost positions, and `defrag_files` which strategically places each file in the appropriate free region based on its size, ensuring gaps are filled efficiently. The checksum calculation was executed by iterating through the blocks and summing the product of each file's position and its ID. This design leverages Rust’s strong type system and the dynamic nature of vectors for both clarity and performance.
</details>
            
<!---DAY9_END-->

<!---DAY10_BEGIN-->

### [⭐️⭐️ Day 10 - Hoof It](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day10/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
In my solution, I utilized a grid-based approach with a recursive depth-first search (DFS) to explore valid hiking trails starting from trailheads, which are positions marked by '0'. I implemented two functions, `find_trails` and `distinct_trails`, that recursively navigate through neighboring cells, validating the height progression criteria and counting reachable '9' positions. A `HashSet` is used to track unique positions visited during the trail exploration, ensuring efficient membership checking and preventing duplicates. The overall scores are computed by iterating through each cell in the grid, applying the respective scoring function, and summing the results to obtain the final score for all trailheads. This design effectively captures the required constraints while maintaining clarity and modularity in the code.
</details>
            
<!---DAY10_END-->

<!---DAY11_BEGIN-->

### [⭐️⭐️ Day 11 - Plutonian Pebbles](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day11/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I implemented a recursive approach to solve the puzzle by defining a function `blink_recursive_count` that takes an input vector of stones, a number of blinks, and a hashmap for memoization. The core of my solution involves applying three transformation rules to each stone—incrementing zero, splitting even-digit stones, and multiplying the others by 2024—encapsulated in the `apply_rules` function. To optimize the computation, I utilize a hashmap to cache results of previously computed states, which prevents redundant calculations during recursion and significantly improves performance, especially as the number of blinks increases. This method captures the recursive nature of the problem while maintaining an efficient handling of the exponential growth of stones.
</details>
            
<!---DAY11_END-->

<!---DAY12_BEGIN-->

### [⭐️⭐️ Day 12 - Garden Groups](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day12/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I implemented a depth-first search (DFS) algorithm to identify distinct regions of garden plots on the input grid, using a stack to track the current position and a HashSet to avoid revisiting cells. For each unvisited cell, I counted the number of plots and recorded the edges that contribute to the region's perimeter, distinguishing between edges that are internal (connecting to the same plant type) and those that are external (adjacent to different plant types or grid boundaries). After computing the regions, I calculated their prices by multiplying the area (number of plots) by the perimeter (derived from the number of unique edges). This approach was efficient given the localized nature of the row and column interactions, and I employed separate methods for price calculations based on either edges or distinct sides touched by the regions.
</details>
            
<!---DAY12_END-->

<!---DAY13_BEGIN-->

### [⭐️⭐️ Day 13 - Claw Contraption](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day13/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I implemented the solution by defining a `ClawMachine` struct that encapsulates the button configurations and prize coordinates, which allows me to directly represent each machine's behavior. I utilized a method to solve a system of linear equations derived from the number of times buttons A and B need to be pressed to reach the prize's coordinates. This involved calculating coefficients for equations corresponding to the X and Y coordinates, checking for valid solutions based on modular arithmetic, and deriving the costs based on the number of button presses. By scanning through each machine and aggregating the costs, I efficiently determined the minimum number of tokens required to win as many prizes as possible.
</details>
            
<!---DAY13_END-->

<!---DAY14_BEGIN-->

### [⭐️⭐️ Day 14 - Restroom Redoubt](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day14/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
I utilized a structured approach by defining a `Robot` struct to encapsulate the position and velocity of each robot, and a `Map` struct to manage a collection of robots within specific dimensions. For simulating robot movement, I implemented a method that updates each robot's position based on its velocity while correctly handling the wrapping behavior using modular arithmetic. To analyze the final distribution of robots, I counted their presence in the four quadrants of the grid by checking their positions relative to the center. The final safety factor was computed by multiplying the robot counts from each quadrant after simulating 100 seconds of movement, leveraging efficient use of vectors for storage and performing calculations in linear time relative to the number of robots.
</details>
            
<!---DAY14_END-->

<!---DAY15_BEGIN-->

### [⭐️⭐️ Day 15 - Warehouse Woes](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day15/mod.rs)
            
<details>
<summary>See explanation</summary>
<br/>
In my solution, I utilized a grid data structure represented by a custom `Grid<WarehouseCell>` type to manage the state of the warehouse, where each cell can hold different types of entities, including walls, vacant spaces, boxes, and the robot. For robotic movements and box manipulation, I implemented a series of directional vectors using a `Vec2d<i32>` type, allowing for clear representation of movement in both horizontal and vertical directions. The algorithm incorporates recursive techniques to identify all boxes that can be pushed in a given direction while checking for collisions with walls. After executing all movements, I calculated the GPS coordinates of remaining boxes using a straightforward formula that combines their positions, thereby efficiently determining the desired result through a systematic traversal of the grid.
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
To solve the problem, I utilized dynamic programming to efficiently compute the number of ways each design can be formed using the available towel patterns. I created a memoization vector to store the number of combinations that lead to each substring of the design. For each position in the design, I iterated backward to check if the substring formed up to the current position exists in the set of available patterns, which I stored in a `HashSet` for O(1) average-time complexity lookups. This allowed me to accumulate counts of possible combinations through the memoization vector, making the overall approach efficient in terms of both time and space.
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
