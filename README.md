<h1 align="center">🦀 Advent of Code 2024 🎄</h1>

<div align="center">

<a href="">![build](https://github.com/mtharrison/advent-of-code2024/actions/workflows/rust.yml/badge.svg)</a>
<a href="">[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)</a>

</div>

My solutions to [AOC 2024](https://adventofcode.com/2024/about) in [The Rust Programming Language](https://www.rust-lang.org/).

<!---DAY1_BEGIN-->
## [⭐️⭐️ Day 1 - Historian Hysteria](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day01/mod.rs)

To solve the puzzle, I first created a function called `parse_input` to read the input data and separate it into two vectors, `lhs` and `rhs`. Each line of input contains two space-separated integers, which I split and parsed into integers, adding them to their respective vectors.

Next, I implemented the `get_distance` function. This function calculates the total distance between the two lists by first sorting both vectors. I then iterated through the sorted lists, computing the absolute difference between each pair of corresponding elements—one from `lhs` and one from `rhs`—and accumulated these differences to get the total distance.

Additionally, I created a `get_similarity` function to count how many times each unique number from the `lhs` appears in the `rhs` vector and sums the product of these counts multiplied by the number itself, which helps to determine the overall similarity score.

The main logic was tested through various unit tests that checked both the distance and similarity calculations against provided example data to ensure accuracy. Finally, I included test cases for the actual puzzle input to print out the results, providing a clear picture of my findings for both the distance and similarity between the lists.


<!---DAY1_END-->

<!---DAY2_BEGIN-->
## [⭐️⭐️ Day 2 - Red-Nosed Reports](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day02/mod.rs)

I began by defining a function to parse the input data into a vector of integer vectors, where each inner vector represents a report's levels as integers. This facilitates easier manipulation of the data later on. 

Next, I implemented the `is_safe` function, which determines if a specific report is safe based on two rules: the differences between adjacent levels must be between 1 and 3, and if required, the function attempts to remove up to two elements to maintain a potential safety while checking for conformity to the difference rule.

To encapsulate the potential for both increasing and decreasing sequences, I created the `check_safe` function. This function examines the report as is and also checks the report in reverse order by comparing it against an inverted version of itself.

Finally, I developed the `count_safe` function, which iterates through all reports, utilizing the `check_safe` function to tally how many reports are deemed safe. For testing, I wrote unit tests to verify the implementation using both example cases and the real input.

Overall, this structured approach of parsing, checking safety and counting allowed me to effectively solve the problem while maintaining clarity and extensibility in the code.


<!---DAY2_END-->

<!---DAY3_BEGIN-->
## [⭐️⭐️ Day 3 - Mull It Over](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day03/mod.rs)

I approached the puzzle by designing a simple state machine to evaluate the instructions extracted from the corrupted memory. First, I created an `Instruction` enum to represent the valid operations: `Mul` for performing multiplications, and `Do` or `Dont` for controlling whether to execute multiplication instructions based on a flag.

To extract the valid multiplication instructions from the input string, I used a regular expression to match the `mul(X,Y)` pattern while ignoring any corrupted characters. I then iterated through these matches, parsing the numbers and creating a vector of `Instruction` instances.

The `Machine` struct maintains the state of execution, including an accumulator to sum the multiplication results and a flag to control whether multiplications should be counted. I implemented an `execute` method to handle different instructions appropriately, updating the accumulator only when allowed.

Finally, the `run_program` function executes the list of instructions in sequence and returns the accumulated sum. I included unit tests to verify the correctness of the logic with example inputs, ensuring it behaves as expected for both part one and part two of the puzzle. Using this structured approach helped me systematically process the corrupted input and obtain the required results efficiently.


<!---DAY3_END-->

<!---DAY4_BEGIN-->
## [⭐️⭐️ Day 4 - Ceres Search](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day04/mod.rs)

To solve the problem, I first interpreted the word search as a grid of characters where I needed to identify the occurrences of the word "XMAS" in multiple orientations: horizontally, vertically, and diagonally, including backward directions. I created a function that processes the entire grid by extracting rows, columns, and diagonal strings. 

For counting occurrences, I utilized the `matches` method to count how many times "XMAS" and its reverse "SAMX" appeared in each extracted string. I looped through all possible search lines, collected them into a single counter, and checked the results against the expected values for both the example and the actual puzzle input.

Additionally, for part two of the puzzle, I examined smaller 3x3 squares of the grid to check if the specific pattern of "XMAS" could be formed. I crafted a helper function to verify a grid against the presence of this pattern, looping through the relevant sections and counting valid configurations. Each aspect was modularized to allow for easy testing, ensuring that both parts of the puzzle were addressed correctly.


<!---DAY4_END-->

<!---DAY5_BEGIN-->
## [⭐️⭐️ Day 5 - Print Queue](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day05/mod.rs)

I started by creating a function `parse_input` to read and separate the page ordering rules from the update paths. I used string manipulation to split the input into two parts: the rules and the paths, then parsed each into a suitable format—`Vec<Edge>` for rules and `Vec<Path>` for the updates.

Next, I implemented the `valid_path` function to check if an update path complies with the given rules. For each path, I iterated through its elements, verifying that each pair of consecutive pages complies with the relevant rules by checking if they exist in the rules list.

To optimize the process of validating paths, I created the `pick_edges` function to gather only the relevant rules for a given path, focusing on rules that involve pages present in the path. This filtered list allowed for efficient topological sorting when needed.

In the test functions, I accumulated the middle page numbers of valid paths by accessing the middle element of the correctly ordered paths and summing them up, first for the example input and then for the actual input.

For paths that weren't valid, I employed the `Graph` structure with a topological sort to determine a valid printing order, again retrieving the middle page from the sorted path.

Finally, I compared the accumulated results with expected values using assertions to ensure the correctness of my approach.


<!---DAY5_END-->

<!---DAY6_BEGIN-->
## [⭐️⭐️ Day 6 - Guard Gallivant](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day06/mod.rs)

I implemented a simulation of the guard's patrol protocol by creating a grid-based world where each cell could represent the guard's current position, obstacles, or vacant spaces. I used an enumeration for directions (up, down, left, right) and modeled the guard's state with a structure containing its position and facing direction.

The core logic revolves around a `step` function, which handles the guard's movement following the defined protocol. Whenever the guard encounters an obstacle, it turns right and remains in its current position; otherwise, it moves forward. I tracked previously visited positions using a hash set to ensure that I counted only distinct cells. 

When setting up the grid from input, I parsed the cells into the appropriate types and located the guard's initial position. During the simulation, if the guard moved outside the grid boundaries, I recorded the result and marked all visited positions before escaping. The final count of distinct positions was returned as the answer. 

This approach efficiently handles the simulation of the guard's patrol while keeping track of visited cells, allowing me to arrive at the solution to the puzzle effectively.


<!---DAY6_END-->

<!---DAY7_BEGIN-->
## [⭐️⭐️ Day 7 - Bridge Repair](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day07/mod.rs)

To solve the puzzle, I started by parsing the input into a usable format, which consists of pairs of test values and their corresponding operands. Each line of the input presents an equation where the test value is separated from the operands by a colon. I used a tuple to represent each equation as a combination of the test value and a vector of integers.

The core of my approach is to check each equation systematically by generating all possible combinations of the operators `+` and `*` (and an additional operator `|` for part 2) between the operands. I utilized the `itertools` crate to employ the `multi_cartesian_product` function, which allows me to explore all combinations of operators for the given operands.

After setting up the operator combinations, I evaluated each equation in a left-to-right manner, as specified, keeping track of the accumulated value. If the computed value matches the test value of that equation, I add this test value to a running total.

The computations were parallelized using the `rayon` crate to improve performance, which is essential given the potentially large input sizes. Finally, I return the total sum of the valid test values from the equations that could hold true based on the applied operators, providing the solution efficiently while ensuring correctness through tests for both example and actual inputs.


<!---DAY7_END-->

<!---DAY8_BEGIN-->
## [⭐️⭐️ Day 8 - Resonant Collinearity](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day08/mod.rs)

I approached the puzzle by creating a structured way to represent the antennas and calculate the unique locations of antinodes based on the frequencies emitted by these antennas. First, I defined a `CellTower` struct to hold the position and frequency of each antenna. The `parse_input` function reads the input data, identifying antennas from the characters in the grid, and records their positions while also determining the dimensions of the grid.

To find the antinodes, I created two separate functions: `find_antinodes_pt1` and `find_antinodes_pt2`. Both functions leverage combinations of antennas, which allows me to easily assess pairs of antennas that share the same frequency. For each pair, I calculated the directional offsets based on their positions, allowing me to determine potential antinode locations that are in line with the antennas.

`find_antinodes_pt1` methodically identifies the unique antinode positions that are exactly twice the distance apart. I applied vector arithmetic to determine these positions while ensuring they remain within the valid grid bounds. This function collects the valid antinode locations and filters them to ensure they are unique using the `unique` method from the `itertools` library.

In `find_antinodes_pt2`, I extended the approach to compute all antinodes that can be generated from pairs of antennas by continuously adding directional offsets until reaching the grid boundaries. I repeated the filtering to ensure only valid positions are collected.

Ultimately, I used a dedicated `antinodes_unique` function to count the unique positions collected from either point calculation method, verifying both parts of the puzzle with appropriate tests. This structured approach allowed me to systematically address the requirements and verify correctness against the examples provided.


<!---DAY8_END-->

<!---DAY9_BEGIN-->
## [⭐️⭐️ Day 9 - Disk Fragmenter](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day09/mod.rs)

I approached the puzzle by implementing a `Disk` struct to represent the disk blocks, consisting of both file blocks and free spaces. I started by parsing the input string into this structured format, where `DiskBlock` can either be a `Free` or a `File` with a specific identifier.

First, I created methods to identify and manage free regions and file blocks on the disk. The `defrag_simple` method efficiently relocates files leftward to fill gaps. It uses two pointers—`first_free_idx` to track the next free position and `last_block` to find the last occupied block. By swapping blocks, I ensure that all free space shifts towards the start.

For a more advanced defragmentation, I implemented `defrag_files`. This method iterates through the files in reverse order based on their IDs, placing each file in the rightmost available space while maintaining their positions as compact as possible. I also updated the tracking of free regions dynamically to accommodate changes after each file's relocation.

After reorganizing the files, I calculated the checksum using the `checksum` method, which multiplies each block's position by its file ID and sums these products, skipping over free space blocks. This calculation gives the final value required for the solution of the puzzle.

I tested my implementation against provided examples to ensure correctness before validating it against the actual input. This structured and methodical approach ensured I could efficiently manipulate the disk data and compute the correct filesystem checksum.


<!---DAY9_END-->

<!---DAY10_BEGIN-->
## [⭐️⭐️ Day 10 - Hoof It](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day10/mod.rs)

I approached the puzzle by modeling the topographic map as a grid and using depth-first search (DFS) to explore potential hiking trails. I defined a function `find_trails`, which recursively navigates adjacent grid positions to identify valid trails starting from a given trailhead (position with height 0). The trails must incrementally increase in height, and I maintained a set of reachable positions to avoid counting duplicates.

I also implemented `distinct_trails` to count how many unique '9' positions could be reached from a trailhead using a similar DFS approach, focusing on counting rather than storing positions. The `trailhead_score` function determines the score for each trailhead by checking if its height is '0' and then using `find_trails` to calculate reachable positions.

To compute the overall score of the map, I created a `map_score` function that iterates through all grid positions, applying the specified scoring function (either `trailhead_score` or `trailhead_rating`). This approach efficiently combines recursive trail exploration with grid traversal to yield the required score summation from all applicable trailheads. Finally, I included tests to confirm the correctness of my implementation against provided examples.


<!---DAY10_END-->

<!---DAY11_BEGIN-->
## [⭐️⭐️ Day 11 - Plutonian Pebbles](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day11/mod.rs)

I approached the problem using a recursive strategy combined with memoization to efficiently compute the number of stones after a specified number of "blinks." To handle the rules that dictate how each stone transforms, I defined separate functions for each rule:

1. **Rule 1** handles the transformation of `0` to `1`.
2. **Rule 2** deals with splitting a stone into two stones if its number has an even number of digits.
3. **Rule 3** multiplies the stone's number by `2024` if none of the other rules apply.

The `apply_rules` function organizes these rules, applying the first applicable one to the input stone. For blinks, I implemented a recursive function called `blink_recursive_count`, which takes the current list of stones and the number of blinks remaining. It uses a hashmap to store results of previously computed states to avoid redundant calculations, significantly improving efficiency.

The input is parsed into a vector of integers, representing the initial state of the stones. I then run simulations for the specified number of blinks, updating the stone collection according to the defined transformation rules. To verify the implementation, I wrote tests that check the output against known results from example cases and the puzzle input. This modular approach ensures that each component is easily testable and the main logic is clear.


<!---DAY11_END-->

<!---DAY12_BEGIN-->
## [⭐️⭐️ Day 12 - Garden Groups](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day12/mod.rs)

I approached the problem by first parsing the garden plot input into a grid structure. I then implemented a method to find all the distinct regions of connected plots of the same type by performing a depth-first search (DFS) starting from each unvisited plot. As I explored the grid, I kept track of the number of plots in each region and collected the edges that contributed to the perimeter.

The edges were stored as a set to eliminate duplicates, allowing me to easily calculate the perimeter later. For the area of a region, I simply counted the number of visited plots during the DFS.

To compute the price of fencing required for each region, I created two separate functions. The first one multiplied the area of a region by the number of unique edges, representing the perimeter efficiently. The second function calculated the perimeter by considering the structural geometry of the edges, particularly focusing on the right angles made by adjacent edges.

Finally, I validated my solution against provided test cases to ensure its correctness and efficiency, achieving both the total price calculation for the first part and a more geometrically accurate evaluation for the second part.


<!---DAY12_END-->

<!---DAY13_BEGIN-->
## [⭐️⭐️ Day 13 - Claw Contraption](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day13/mod.rs)

I used a structured approach to tackle the problem of determining the minimum number of tokens needed to win prizes from claw machines. First, I defined a `ClawMachine` struct to represent each machine's button movements and prize locations, extracting the necessary data using regular expressions for efficient parsing. This allowed me to create instances of `ClawMachine` from input strings.

In the `solve` method, I established a system of linear equations to compute how many times to press each button (A and B) to precisely reach the prize's coordinates. By equating coefficients, I derived the necessary formulas to calculate the number of presses, ensuring to check the validity of the results, such as avoiding division by zero and ensuring that the results are integers.

To find the total cost of token usage, I implemented a `cost` method that calculates the token expenditure based on the number of button presses derived from the `solve` function. By iterating through all claw machines, I summed the costs to derive the overall minimum tokens required.

Finally, I validated my approach through unit tests, comparing the implementation against known example values and ensuring the accuracy of the parsing and calculations against the real puzzle input. This structured way of addressing the problem helped me efficiently tackle and solve it.


<!---DAY13_END-->

<!---DAY14_BEGIN-->
## [⭐️⭐️ Day 14 - Restroom Redoubt](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day14/mod.rs)

I began by defining two main structures, `Robot` and `Map`, to encapsulate the properties of the robots and the environment they inhabit. Each `Robot` contains its position and velocity represented by a 2D vector, while the `Map` holds a collection of robots and the dimensions of the grid.

Next, I implemented the `step` method within the `Map` that updates the position of each robot based on its velocity. To account for edge wrapping, I used the modulo operator to ensure that if a robot moves beyond the grid boundaries, it reappears on the opposite side. This mimics the teleportation feature outlined in the puzzle.

To simulate the movement over time, I created the `step_n_times` method, which calls the `step` method repeatedly for the specified number of iterations. After simulating for 100 seconds, I needed to determine the distribution of robots across four quadrants. To achieve this, I implemented the `count_quadrants` function, which updates a count for each quadrant based on the robots' final positions.

The safety factor was computed by multiplying the number of robots in each quadrant, a task handled by the `safety_factor` method. Additionally, I created a grid representation of the map for visualization and included a heuristic to evaluate the regularity of the robots’ distribution through the `regularity_score` method.

Finally, I used the `parse_input` function with a regular expression to read and parse the input data, creating `Robot` instances as required. The tests ensured correct calculations for both example and actual inputs, validating the implementation against the specified conditions in the puzzle description.


<!---DAY14_END-->

<!---DAY15_BEGIN-->
## [⭐️⭐️ Day 15 - Warehouse Woes](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day15/mod.rs)

I created a simulation of a warehouse where a robot moves around attempting to push boxes based on given instructions. I structured my code using a `Warehouse` struct that encapsulated a grid representing the warehouse and the position of the robot. The warehouse grid consists of different cell types, including walls, vacant spaces, and boxes, with a special representation for the robot itself.

To handle the pushing of boxes, I implemented recursive functionality to gather all boxes that can be pushed in a certain direction. The robot checks its desired movement and, depending on the cell encountered, decides whether to simply move, push a box, or do nothing if blocked by a wall. For boxes, I correctly handled their dual representation (left and right halves) to ensure accurate positioning during movements.

After simulating all movements, I calculated the GPS score by iterating through the grid and summing the coordinates of all box positions according to the specified formula. I also incorporated functionality to upscale the warehouse when necessary, effectively doubling its size and resizing the internal representation.

In my tests, I verified the correctness of the simulation against both provided examples and my own inputs, ensuring that the expected outcomes matched actual results. This meticulous implementation ensured that the robot's movements were accurately represented, and the final GPS score of the boxes was computed correctly.


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

I approached the puzzle using a dynamic programming technique to determine how many ways each design can be constructed using the available towel patterns. I defined the function `num_ways`, which takes a design string and a list of patterns, and it initializes a memoization vector to store the number of ways to construct substrings of the design.

For each position in the design, I iterated backward through previous positions to check if the substring from those earlier positions to the current position matches any of the available patterns. If a match is found, I updated the memoization array by adding the count of ways to form the substring up to the earlier position.

The `parse_input` function splits the input into rules and designs, allowing me to prepare the data for processing. In the tests, I used this logic to filter through the designs and count how many can be generated with the available patterns. By summing the valid pathways, I was able to solve the problem efficiently. 

Overall, this method ensures that I check all possible combinations of patterns that fit into the designs while reusing computed results to optimize performance.


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
