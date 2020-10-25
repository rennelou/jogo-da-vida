import numpy as np

size = 5
maskSize = 3
maskCenterIndex = 1


matStates = np.array([[0, 0, 0, 0, 0],
                      [1, 1, 0, 0, 0],
                      [0, 1, 0, 0, 0],
                      [1, 1, 0, 0, 0],
                      [1, 1, 0, 0, 0]])

def transition(mat):
    count = np.count_nonzero(mat == 1)
    if (mat[maskCenterIndex][maskCenterIndex] == 1):
        return 1 if count == 2 or count == 3 else 0
    else:
        return 1 if count == 3 else 0

def tick(mat):
    newMatState = np.zeros((size, size))
    for i, line in enumerate(mat):
        for j,_ in enumerate(line):
            limitedIndex = lambda index: index-1 if index > 0 else 0
            newMatState[i][j] = transition(mat[limitedIndex(i):i+2, limitedIndex(j):j+2])

    return newMatState

print("Initial states")
print(matStates)
print("")

for i in range(10):
    matStates = tick(matStates)
    print(f'Time {i}')
    print(matStates)
    print("")