## Big O

Given enough time, any developer can solve any problem. One thing that ends up being important is the speed and efficientcy with how that problem was solved.

Good code is subjectively:
- readability
- scalability


Big O is used to measure time complexity using the alg O(x), where x is the number of operations.

O(n) is linear time. As the size increases, the time complexity increases at the same rate.

O(1) is constant time. No matter how the size increases, time complexity will stay the same.
Even if there are 500 operations, as long as the number of operations is a constant as size grows, this can always be demonstrated as O(1).

```javascript
// What is the Big O of the below function? (Hint, you may want to go line by line)
function funChallenge(input) {
  let a = 10; // O(1)
  a = 50 + 3; // O(1)

  for (let i = 0; i < input.length; i++) { // O(n)
    anotherFunction(); // O(n)
    let stranger = true; // O(n)
    a++; // O(n)
  }
  return a; // O(1)
}
//  3 + 4n
// O(3 + 4n) = O(n)
```

### Rule 1 - Worst Case
Big O only cares about worst case. That being, assume that what we are looking for will always be at the end of the list. 
Afuncitonlways consider the worst case scenario, aka the slowest case scenario. Even if you break when a condition is met, you must calculate big O as if all iterations and checks were exhausted.

### Rule 2 - Remove Constants
We dont care about the O(1) assignments.
So, O(1 + n/2 + 100) just simplifies to O(n).

### Rule 3 - Different Terms for Inputs
If there are two different inputs, they will have different term.

So a function with two array inputs, it could be O(a + b).
If they are nested, it would be quadratic time like O(n ^ 2) but with different terms: O(a * b)

### Rule 4 - Drop Non Dominants
This means we care about the most important term, so O(n + n ^ 2) becomes O(n ^ 2).

### Space Complexity
What causes this?
- variables
- data structs
- function calls
- allocatioins
