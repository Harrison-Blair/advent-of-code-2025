from pathlib import Path

def read_file(input):
  list = []

  with open(input, 'r') as file:
    for line in file:
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

if __name__ == "__main__":
  script_location = Path(__file__).parent
  filePath = script_location / "input.txt"

  print("\nReading from " + str(filePath) + "...")
  manifold = read_file(filePath)

  p1 = part_one(manifold)
  print("\nPart-1 [RESULT] Total Splits : " + str(p1))