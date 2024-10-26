# scalc

a fun little project for me to approximate numbers.

currently implemented:

## π, approx. 3.14159
- arclength method
    - start with t=0, x=1, y=0
    - apply x'=-y, and y'=x, and integrate until x≤y
    - the t-value at this point will be pretty close to π/4
- area: approximately integrates sqrt(1-x²) from 1 to 1/√2, doubles, and subtracts ~1/2.
    - it's all done with integers, so none of that is precise: really, it starts at (0,n), and integrates ~sqrt(n² - x²)until x>y.
    - double that integral, subtract the final x-value squared, divide by n², and you get something like π/4.
- a funny sum that i keep hearing about
    - 1 - ⅓ + ⅕ - ⅐ + ... = π/4
    - that's it that's the whole algorithm

## Euler's number, e, approx. 2.71828
- classic sum
    - you know it, you love it, it's the sum of the reciprocals of the factorials of the natural numbers
    - 1 + 1 + ½ + ⅙ + the rest of them
- interest method
    - Euler's number is often presented to classes in this format
    - 100% annual interest, compounded more and more and more frequently, tends towards an "effective interest" of ~178%.
    - The limit of (1+1/n)ⁿ as n tends to infinity is e

## The natural logarithm of 2, approx. 0.69314
- taylor series!
    - the taylor series for ln(1+x) gives 1 - ½ + ⅓ - ¼ + ⅕ - ⅙ + ... = ln(2)
- logarithm properties
    - start with ln(2). 2 < e, so go to ln(2²)/2 = ln(4)/2
    - this becomes ½ + ln(4/e)/2
    - and then it keeps going like that
