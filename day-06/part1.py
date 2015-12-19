from __future__ import print_function
import fileinput
import re
import sys

def compact(lightList):
    last = None
    newList = []
    for streak in sorted(lightList):
        if last is not None:
            if streak[2] == last[2]:
                # Repeating state, merge
                last = (last[0], streak[1], last[2])
                continue
            else:
                newList.append(last)

        last = streak

    newList.append(last)

    return newList

def colsum(lightList):
    return sum(int(state) * (stop - start + 1) for start, stop, state in lightList)

lights = []
for x in range(1000):
    lights.append([(0, 999, False)])

for line in fileinput.input():
    match = re.search(r"^(toggle|turn (on|off)) (\d+),(\d+) through (\d+),(\d+)$", line)

    xStart = int(match.group(3))
    yStart = int(match.group(4))

    xEnd = int(match.group(5))
    yEnd = int(match.group(6))

    command = match.group(1)

    for x in range(xStart, xEnd + 1):
        newList = []
        for start, end, state in lights[x]:
            if not start <= yStart  <= end and not yStart <= start <= yEnd:
                # Block not in range, skip
                newList.append((start, end, state))
                continue

            if start < yStart:
                # Split the block at the start
                newList.append((start, yStart - 1, state))
                start = yStart

            if end > yEnd:
                # Split the block at the end
                newList.append((yEnd + 1, end, state))
                end = yEnd

            if "toggle" in command:
                newList.append((start, end, not state))
            elif "on" in command:
                newList.append((start, end, True))
            else:
                newList.append((start, end, False))

        lights[x] = compact(newList)

total = sum(colsum(x) for x in lights)

print(total)
