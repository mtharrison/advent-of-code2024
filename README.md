<h1 align="center">🦀 Advent of Code 2024 🎄</h1>

<div align="center">

<a href="">![build](https://github.com/mtharrison/advent-of-code2024/actions/workflows/rust.yml/badge.svg)</a>
<a href="">[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)</a>

</div>

My solutions to [AOC 2024](https://adventofcode.com/2024/about) in [The Rust Programming Language](https://www.rust-lang.org/).

<!---BEGIN-->
## [Day 1 - Historian Hysteria](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day01/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ (<1ms)

I started by creating a function to parse the input, separating the location IDs into two vectors: `lhs` for the left list and `rhs` for the right list. I read the input line by line, splitting each line into its respective parts and converting them to integers.

Next, to compute the total distance, I implemented the `get_distance` function. This function first sorts both vectors to allow for direct pairing of the corresponding elements. Then, I iterated through the indices, calculating the absolute difference between each pair of values and accumulating the total distance.

For the second part of the puzzle, I developed the `get_similarity` function, which tallies how often each number in the left list appears in the right list. For each unique number in `lhs`, I used the `filter` method to count its occurrences in `rhs`, multiplying this count by the number itself to contribute to the overall similarity score.

Finally, in the test module, I wrote tests to validate both functions against example data and the actual input file, ensuring the correctness of my implementation. This systematic approach helped me efficiently reconcile the two lists, yielding the required total distances for the puzzle.
## [Day 2 - Red-Nosed Reports](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day02/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ (<1ms)

I started by defining a function `parse_input` that takes the raw input string and converts each line into a vector of integers, which represent the levels in each report. This provides a structured format for further analysis. 

Next, I implemented the `is_safe` function, which checks if a specific report meets the safety criteria. It iterates through the report and calculates the difference between adjacent levels. If a difference falls outside the allowable range (1 to 3), I handle the situation based on the `tolerance` flag. If `tolerance` is false, the report is deemed unsafe. If true, I create clones of the vector with the problematic levels removed to check the safety of the adjusted reports recursively.

For broader checks, I created the `check_safe` function, which determines if a report is safe by checking the original order and the reverse order of levels. This ensures that all potential cases of increasing or decreasing sequences are considered.

Finally, I wrote a `count_safe` function that counts how many reports are safe by iterating over each report and using the `check_safe` function. This function initializes a count and aggregates results as it evaluates each report.

In the tests, I validated the implementation with example reports and then tested against the actual puzzle input, logging the safe report counts accordingly. This structured approach ensured that I systematically checked both safety criteria while accommodating varying levels of tolerance.
## [Day 3 - Mull It Over](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day03/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ (<1ms)

I started by defining an `Instruction` enum to represent the different commands we need to handle, specifically the `Mul` operation along with `Do` and `Dont` for controlling execution. The `Machine` struct manages the execution context, including an accumulator to store the sum of the multiplication results.

To extract the valid multiplication instructions from the corrupted input, I utilized a regular expression to find patterns that matched the `mul(X,Y)` format. This regex ensures that we accurately isolate valid multiplications while ignoring corrupt segments featuring invalid characters. 

After extracting the instructions, I translated them into the enum types. For each valid `mul` instruction found, I parsed the two numbers and created a corresponding `Instruction::Mul` variant. For the `do()` and `don't()` commands, I handled them by updating the machine state to enable or disable instruction execution based on whether I wanted to evaluate those commands.

In the `run_program` method, I iteratively executed the extracted instructions. For each `Mul` instruction, if the machine is enabled, I updated the accumulator with the result of the multiplication. Finally, I returned the accumulated result after processing all valid instructions.

The implementation also included tests for both the example cases and the actual puzzle input, ensuring that the results were as expected. This approach leads to a clear and effective solution, maintaining focus on the valid operations while navigating through the corrupted data.
## [Day 4 - Ceres Search](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day04/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ (<1ms)

To solve the problem of counting the occurrences of the word "XMAS" in a grid, I took a systematic approach to search through various orientations of the word. First, I implemented a function that checks for the word in horizontal, vertical, and diagonal lines in the grid. The function collects strings corresponding to these orientations into a vector.

In my tests, I ran through the grid several times, transforming rows, columns, and diagonals into strings. For each string, I utilized the `matches` function to count instances of "XMAS" and its reverse "SAMX", as the word can appear in both configurations due to the nature of words being reversible in this search format.

In addition to counting occurrences for the first part of the puzzle, I also designed the solution to capture additional checks in 3x3 squares for the second part. This involved iterating over the grid to extract sub-grids and applying the previous check. This way, I ensured that I counted both the individual occurrences of "XMAS" in various orientations as well as the specific patterns in smaller grid sections. Ultimately, I validated the solutions against example cases to confirm accuracy.
## [Day 5 - Print Queue](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day05/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ (<1ms)

To solve the puzzle, I first parsed the input to separate the page ordering rules and the updates. I defined the `Edge` type to represent each order constraint, and the `Path` type to collect the pages in each update. For parsing, I split the input string into two parts based on double newlines, extracting the rules and paths accordingly.

Next, I implemented the `valid_path` function to check if a given path adheres to the ordering rules. This function iterates through the pages in the path and verifies that each adjacent pair complies with the defined constraints by searching for matching rules in the list.

To address the need for considering relevant rules for each update, I created the `pick_edges` function, which filters the rules to include only those applicable to the current path. This allows the construction of a subgraph based on the constraints that remain after considering the pages in that specific update.

Finally, I computed the result for correctly-ordered updates by accumulating the middle page number from each valid path. For invalid paths, I constructed a graph from the relevant edges and applied topological sorting to find a valid order, again obtaining the middle page number. I ran tests for both the example input and the actual input, asserting the correctness of my implementations by checking against expected results.
## [Day 6 - Guard Gallivant](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day06/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ 0.06s

I first defined the necessary enums and structs to represent the guard's state, the world, and the cells in the grid. The `Direction` enum encapsulated the possible directions the guard could face, while the `Cell` enum represented the types of cells in the world, including the guard, vacant spaces, obstacles, and visited positions. The `GuardState` struct kept track of the guard's current position and direction.

In the `World` struct, I implemented a method called `step()` to simulate the guard's movement based on the described rules. When the guard encounters an obstacle directly in front of it, I updated its direction by turning right. If the path was clear, the guard moved forward and marked the previous position as visited.

The core of the simulation happened in the `play()` method, which maintained a loop where the guard continued to step until it either left the mapped area or entered a repeating situation (loop). I utilized a `HashSet` to track both positions visited and the guard's state (position and direction) to detect loops effectively.

To initialize the world from the input string, I implemented the `From<String>` trait for the `World` struct. This allowed me to parse the input into a grid of cells, identifying the initial position and direction of the guard.

In the tests, I confirmed the correctness of my implementation using example inputs and the actual puzzle input, checking that the number of distinct positions visited matched the expected results. The modular structure and clear separation of concerns in the code facilitated easy maintenance and straightforward debugging.
## [Day 7 - Bridge Repair](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day07/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ 0.09s

I approached this puzzle by first parsing the input into a manageable format. Each line of input is transformed into a tuple consisting of a target result and a vector of operands. This allows for easy manipulation of the data later on.

To determine which equations could potentially be valid, I implemented a method that utilizes a parallel approach to evaluate all combinations of operators between the operands. I employed the `itertools` crate to generate all possible configurations of the operators (`+` and `*`) for the given operands. Since the number of operands can vary, this is handled dynamically based on the input.

Inside the evaluation loop, I maintain an accumulator to apply the operations in a left-to-right manner, as specified. For each combination of operators, I check if the evaluated value matches the target result. If it does, the target result is added to a running total. This process is done in parallel for efficiency, leveraging the `rayon` library to handle concurrent computations across multiple threads.

Lastly, I run tests on both example inputs and actual inputs to ensure the implementation is correct and returns the expected sums of valid test values. The overall structure is designed to be efficient while adhering to the constraints of operator precedence and operand order.
## [Day 8 - Resonant Collinearity](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day08/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ (<1ms)

I approached the problem by first defining a structure, `CellTower`, to represent each antenna's position and frequency. I processed the input to gather these antennas while also calculating the map dimensions. To find the antinodes, I utilized combinations of pairs of antennas, filtering to ensure they shared the same frequency. 

In the `find_antinodes_pt1` function, I computed the positions of potential antinodes based on the relative distances of antennas, specifically looking for points that would be positioned at half the distance of one antenna from the other. I then filtered these positions to remove any that fell outside the defined map bounds.

For the second part, `find_antinodes_pt2`, I extended this by generating antinode positions further away from each antenna until I hit the map edges, effectively capturing all possible valid antinodes from each antenna pair.

In both functions, I used a utility to collect unique antinode positions, ensuring I counted only distinct locations. Finally, I incorporated tests to validate the correctness of my functions against provided examples and my puzzle input, confirming the expected results for both parts of the challenge.
## [Day 9 - Disk Fragmenter](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day09/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ 0.45s

I approached the puzzle by creating a structured representation of the disk using a `DiskBlock` enum to differentiate between free space and file blocks. This allowed for a clear and understandable manipulation of the disk's state. I implemented a `Disk` struct to hold a vector of these blocks along with indices to track the first free space and the last block.

To compact the files, I designed two methods: `defrag_simple` and `defrag_files`. The `defrag_simple` method performs a basic compaction by swapping blocks until all files are brought together in order, while ensuring free spaces are pushed to the end. `defrag_files` refines this by identifying the largest gaps and moving files from the end to the nearest available free space, preserving file order.

After compacting, I calculated the filesystem checksum by iterating through the blocks. For each block, if it contained a file, I multiplied its position by the corresponding file ID and summed these values.

In the parsing phase, I constructed the disk from a string representation of the input, alternating between file lengths and free space lengths. This ensured that the representation was accurate and straightforward for performing operations later.

Overall, my code effectively compacted the disk and computed the required checksum through structured data management and targeted algorithms for file movement.
## [Day 10 - Hoof It](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day10/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ (<1ms)

I approached the puzzle by first defining a recursive function, `find_trails`, which explores the grid starting from a given position, looking for valid hiking trails that go from height 0 to height 9, increasing by exactly 1 at each step. This function employs a depth-first search strategy, utilizing a `HashSet` to track the reachable positions at height 9.

For calculating scores from trailheads, I created two distinct functions: `trailhead_score` and `trailhead_rating`. The `trailhead_score` function uses `find_trails` to count the number of reachable height 9 positions from trailheads at height 0, while the `trailhead_rating` function counts the distinct trails directly.

The `map_score` function iterates over each cell in the grid, applying the score function to gather scores for all trailheads. I ensured the solution was efficient by reusing the grid data structure and processing the input in a structured manner. Finally, I validated the implementation with unit tests against provided examples and the actual puzzle input, confirming the expected outcomes.
## [Day 11 - Plutonian Pebbles](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day11/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ 0.02s

To solve the puzzle, I designed a system of rules governing how the stones change each time they blink. First, I implemented three rules as separate functions: 

1. The first rule replaces a stone with `0` by `1`.
2. The second rule splits a stone with an even number of digits into two separate stones, while the third rule multiplies the stone's value by `2024` if neither of the other rules apply. 

For the processing, I created an `apply_rules` function, which takes a stone's value and returns a vector of new stones based on the applicable rules. Next, I tackled the challenge of handling multiple iterations of blinking, which could lead to a substantial number of stones. I utilized a recursive function, `blink_recursive_count`, applying memoization via a `HashMap` to store the results of previous computations, thus optimizing the process and avoiding redundant calculations.

A helper function, `blink_count`, initializes this recursion by setting up the hashmap for tracking calculated values. Finally, I wrote a `parse_input` function to handle the input and convert it into the necessary format. My testing confirmed the solution against example and actual input, ensuring correctness across different scenarios.
## [Day 12 - Garden Groups](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day12/mod.rs)
- Part 1: ⭐️ 0.01s
- Part 2: ⭐️ 0.01s

I started by defining a structure to represent the edges and regions of the garden plots, encapsulating key information such as the number of plots and their edges as distinct data types. I then created a function to parse the input into a grid format.

To identify the regions of plants, I used a depth-first search algorithm implemented via a stack. As I traversed the grid, I marked the visited plots and counted both the number of plots within a region and the edges contributing to its perimeter. I managed edges using a `HashSet` to avoid duplicates.

After gathering all regions, I computed the cost of fencing each region using two distinct pricing strategies. The first approach involves calculating the total price by multiplying the area of each region by the number of unique edges. In the second approach, I refined the edge counting to consider only the sides exposed to the outside, leveraging geometrical insights about the number of vertices. 

Lastly, I thoroughly tested both parts of the puzzle to ensure correctness against the provided examples, validating that my calculations for total fencing prices aligned with expected results.
## [Day 13 - Claw Contraption](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day13/mod.rs)
- Part 1: ⭐️ 0.02s
- Part 2: ⭐️ 0.02s

I approached the problem by defining a `ClawMachine` struct that encapsulates the movement costs and prize locations for each machine. I started by using regex to parse the machine behavior and prize coordinates from the input strings. This allows me to extract the movements associated with buttons A and B, as well as the prize's position.

To determine the number of times each button needs to be pressed, I formulated a method to solve a set of linear equations based on the cost constraints of the buttons. Specifically, I used symbolic manipulation to derive the necessary counts of button presses that would align the claw over the prize coordinates, given a variable offset to account for potential adjustments.

In terms of cost computation, I defined a function to calculate the total token expenditure based on the button press counts returned by the equation solver. If a valid solution exists, the costs are summed; otherwise, it returns zero. 

Finally, I parsed the input into a vector of `ClawMachine` instances and iterated over these to compute the total cost required to win as many prizes as possible, based on the solutions derived for pressing the buttons up to a maximum limit. This structured approach ensures that I efficiently find the optimal solutions for each machine while adhering to the specified constraints.
## [Day 14 - Restroom Redoubt](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day14/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ 0.15s

I structured my solution around the concept of a `Robot` and a `Map` that holds multiple robots. The `Robot` struct contains its current position and velocity, both represented using a `Vec2d` for easy arithmetic operations. The `Map` struct tracks all robots and its dimensions.

To model the movements of robots over time efficiently, I implemented a `step` method that updates each robot's position based on its velocity while also applying the wrapping behavior at the edges of the map. I created a `step_n_times` function to allow for simulating the robot's movements over a specified number of seconds (in this case, 100).

After simulating the movements, I needed a method to analyze the robots' final positions by dividing the map into quadrants and counting the number of robots in each. The `count_quadrants` function categorizes robots into one of the four quadrants and increments their respective counts. To compute the safety factor, I multiplied the counts from all four quadrants together.

For input parsing, I used a regular expression to extract the positions and velocities of robots from the puzzle input format. The `Grid` struct was also utilized to produce a visual representation of the robot positions.

Unit tests ensured that both the example and actual inputs produced the expected results, validating the correctness of my implementation.
## [Day 15 - Warehouse Woes](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day15/mod.rs)
- Part 1: ⭐️ (<1ms)
- Part 2: ⭐️ (<1ms)

I approached the problem by first defining the core data structures that represent the warehouse and its components. I created an enum `WarehouseCell` to represent different types of cells, such as walls, vacants, boxes, and the robot. The `Warehouse` struct encapsulates a grid of these cells along with the robot's position.

To handle the robot's movements, I implemented a series of methods in the `Warehouse` struct. The `step` method checks the cell in front of the robot and determines how to move based on whether the next cell is empty, contains a box, or is a wall. If a pushable box is encountered, I recursively gather all boxes in the push direction using `get_pushable_boxes`, ensuring proper movement even when boxes are adjacent to each other.

I incorporated distinct movement logic for vertical and horizontal pushes to account for the box types, which are treated as separate left and right halves. The method `move_pushable_boxes` orchestrates this process depending on the direction of movement.

After processing all moves, I calculated the final score by defining the `gps_score` method, which sums GPS coordinates of boxes based on their positions within the grid. To parse the input data, I split it into the warehouse configuration and the movement instructions. The instructions were converted into vector movements representing their direction (up, down, left, right).

Finally, I implemented tests using example inputs to validate both the initial and upsized warehouse configurations, ensuring that the robot's movements and the resulting scores were computed correctly across various scenarios. This structured approach allowed me to effectively simulate the robot's behavior and accurately compute the desired sum of the boxes' GPS coordinates.
## [Day 16 - Reindeer Maze](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day16/mod.rs)
- Part 1: 
- Part 2: 


## [Day 17 - Chronospatial Computer](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day17/mod.rs)
- Part 1: 
- Part 2: 


## [Day 18 - RAM Run](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day18/mod.rs)
- Part 1: 
- Part 2: 


## [Day 19 - Linen Layout](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day19/mod.rs)
- Part 1: ⭐️ 0.01s
- Part 2: ⭐️ 0.01s

I approached the problem by implementing a dynamic programming solution to count the number of ways to form each design using the available towel patterns. I defined the function `num_ways`, which takes a design string and a list of patterns as input. Within this function, I initialized a memoization array, `memo`, where `memo[i]` stores the number of ways to create the substring of the design from the start up to the `i-th` character.

As I iterated through each character of the design, I checked all possible substrings that end at the current position. For each substring, I checked if it exists in the set of available patterns using a `HashSet`, which allows for efficient look-up times. If a substring matches a pattern, I incremented `memo[i]` by `memo[j]`, where `j` is the start index of the substring. 

The function `parse_input` splits the input into available patterns and desired designs, allowing easy parsing from the given input format. I also included tests to validate my implementation against both example inputs and the final puzzle input, ensuring correctness while counting the number of valid designs. Overall, I effectively utilized dynamic programming and a hash set to simplify the solution and optimize performance.
## [Day 20 - 🔒](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day20/mod.rs)
- Part 1: 
- Part 2: 


## [Day 21 - 🔒](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day21/mod.rs)
- Part 1: 
- Part 2: 


## [Day 22 - 🔒](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day22/mod.rs)
- Part 1: 
- Part 2: 


## [Day 23 - 🔒](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day23/mod.rs)
- Part 1: 
- Part 2: 


## [Day 24 - 🔒](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day24/mod.rs)
- Part 1: 
- Part 2: 


## [Day 25 - 🔒](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day25/mod.rs)
- Part 1: 
- Part 2: 

<!---END-->
