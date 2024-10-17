# scalc

a fun little project for me to approximate numbers.

currently implemented:

## π, approx. 3.14159
- arclength, a differential equation-based method (dx/dt=-y, dy/dt=x, start with x=1 and y=0, go until x<0, t will be ~π/2)
- area: approximately integrates sqrt(1-x²) from 1 to 1/√2, doubles, and subtracts ~1/2.
    - it's all done with integers, so none of that is precise: really, it starts at (0,n), and integrates ~sqrt(n² - x²)until x>y.
    - double that integral, subtract the final x-value squared, divide by n², and BAM! approximately π/4.
- ???? more to come

## possible additions: euler-mascheroni constant (γ, approx. 0.57721), euler's number (e, approx. 2.71828)
