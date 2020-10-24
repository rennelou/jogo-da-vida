import numpy as np

size = 3
maskSize = 3

matStates = np.array([[0, 0, 0],
                      [1, 1, 0],
                      [0, 1, 0]])

def transition(mat):
    count = np.count_nonzero(mat == 1)
    return 1 if count >= 3 else 0

def tick(mat):
    newMatState = np.zeros((size, size))
    for i, line in enumerate(mat):
        for j,_ in enumerate(line):
            limitedIndex = lambda index: index-1 if index > 0 else 0
            newMatState[i][j] = transition(mat[limitedIndex(i):i+2, limitedIndex(j):j+2])

    return newMatState

print(matStates)
matStates = tick(matStates)
print(matStates)