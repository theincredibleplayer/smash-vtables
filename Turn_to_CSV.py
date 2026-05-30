fighters = []
with open('Vtable/weapon_vtable_documentation.txt', 'r') as file:
    content = file.readlines()
    fighter = ["WEAPON"]
    for line in content:
        line = line.strip()
        if ":" in line and line[0].isdigit():
            line = line.split(':')[1].strip()
            line = line.split('//')[0].strip()
            fighter.append(line)
    fighters.append(fighter)

for i in fighter:
    print(i)

with open('Vtable/weapon_vtable_pointers.txt', 'r') as file:
    content = file.readlines()
    fighter = []
    for line in content:
        line = line.strip()
        if len(line) == 0:
            continue
        #if ":" not in line:
        if "::" in line:
            fighters.append(fighter)
            fighter = [line]
        else:
            pointer = line.split(':', 1)[-1].strip()
            fighter.append(pointer)
    fighters.append(fighter)


fighters.pop(1)


fighters = [list(row) for row in zip(*fighters)]

import csv
with open('output.csv', 'w', newline='') as f:
    writer = csv.writer(f)
    writer.writerows(fighters)
