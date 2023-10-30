with open("missing_moves", "r") as f:
     lines = f.readlines()
     for (i, line) in enumerate(lines):
             if i%2 == 0:
                     if lines[i+1] != line:
                             print(line, lines[i+1])

