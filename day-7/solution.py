import copy

from collections import deque

from pathlib import Path

def read_file(input):
  list = []

  with open(input, 'r') as file:
    for line in file:
      line = line.rstrip('\n')
      l = []
      for c in line:
        l.append(c)
      list.append(l)

  return list

def part_one(manifold: list):
  total_splits = 0

  for i in range(len(manifold)):
    line: list  = manifold[i]
    if i == 0:
      manifold[i + 1][line.index('S')] = '|'
      continue
    if i + 1 == len(manifold):
      continue

    for j in range(len(line)):
      c: str = line[j]
      if c == '|':
        if manifold[i + 1][j] == '^':
          manifold[i + 1][j - 1] = '|'
          manifold[i + 1][j + 1] = '|'
          total_splits += 1
        else:
          manifold[i + 1][j] = '|'

  return total_splits

def part_two(manifold: list):
  rows = len(manifold)
  cols = len(manifold[0])
  
  # Find S
  start_r, start_c = 0, 0
  for r in range(rows):
    if 'S' in manifold[r]:
      start_r = r
      start_c = manifold[r].index('S')
      break
      
  # dp[r][c] stores the number of timelines resulting from a particle reaching (r, c)
  dp = [[0 for _ in range(cols)] for _ in range(rows)]
  
  # Base case: The bottom row. 
  for c in range(cols):
    dp[rows - 1][c] = 1
    
  # Fill table bottom-up
  for r in range(rows - 2, -1, -1):
    for c in range(cols):
      # If the cell below is a splitter '^', the particle splits
      if manifold[r + 1][c] == '^':
        left = dp[r + 1][c - 1] if c - 1 >= 0 else 0
        right = dp[r + 1][c + 1] if c + 1 < cols else 0
        dp[r][c] = left + right
      else:
        # Otherwise, it continues straight down
        dp[r][c] = dp[r + 1][c]
        
  return dp[start_r][start_c]

if __name__ == "__main__":
  script_location = Path(__file__).parent
  filePath = script_location / "input.txt"

  print("\nReading from " + str(filePath) + "...")
  manifold = read_file(filePath)

  m2 = copy.deepcopy(manifold)
  p1 = part_one(manifold)
  print("\nPart-1 [RESULT] Total Splits : " + str(p1))

  p2 = part_two(m2)
  print("\nPart-2 [RESULT] Total Timelines : " + str(p2))