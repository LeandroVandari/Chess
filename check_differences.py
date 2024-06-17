differences = []
equal = []

with open("missing_moves", "r") as f:
        lines = f.readlines()
        for (i, line) in enumerate(lines):
                if i%2 == 0:
                        if lines[i+1] != line:
                                differences.append(f"\t{line.strip()}\n\t{lines[i+1].strip()}\n")
                        else:
                                equal.append(f"\t{line.strip()}\n\t{lines[i+1].strip()}\n")


print("Differences:", *differences, sep="\n")
print("Equal:", *equal, sep="\n")