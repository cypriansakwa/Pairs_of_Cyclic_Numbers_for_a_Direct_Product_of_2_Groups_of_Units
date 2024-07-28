This Rust code consists of multiple functions that work together to find and output pairs of cyclic numbers for an group of units Z*_n x Z*_m up to a certain limit. 
Here's an overview of the functions and their responsibilities:
1. GCD Calculation computes the greatest common divisor of two numbers.
2. Coprime Check uses the GCD to check if two numbers are coprime.
3. Euler's Totient Function calculates the totient function which is essential in determining cyclic numbers.
4. Cyclic Number Check determines if a number is cyclic based on certain properties, including primality.
5. Prime Check determines if a number is prime, used in checking cyclic properties.
6. Finding Cyclic Pairs searches for pairs of cyclic numbers up to the specified limit and checks if their totient values are coprime.
7. Main Execution executes the program, printing pairs of cyclic numbers within the given limit.

   git clone https://github.com/cypriansakwa/Pairs_of_Cyclic_Numbers_for_a_Direct_Product_of_2_Groups_of_Units.git
   cd Pairs_of_Cyclic_Numbers_for_a_Direct_Product_of_2_Groups_of_Units
