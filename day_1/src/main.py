import numpy as np

input = np.loadtxt('./input', dtype=int)
input = np.sort(input, axis=0)
input = input.T
solution = np.sum(np.abs(input[0] - input[1]))

print(f"distance: {solution}")

# Part 2

sum = 0
for i in input[0]:
    sum += i * np.sum(input[1] == i)

print(f"Calculated similarity score: {sum}")

