import re

# TODO: Fix wrong answer
with open("input.txt", "r") as f:
    puzzle = f.read()

puzzle = puzzle.split("\n")

def check_line(previous: str, current: str, next: str):
    subbed = re.sub('[^0-9]', '.', current)
    filtered = list(filter(lambda x: x.isdigit(), subbed.split(".")))

    parts = []
    for number in filtered:
        starting_index = current.index(number)
        ending_index = starting_index + len(number)

        bounded_left = starting_index - 1 if starting_index > 0 else 0
        bounded_right = ending_index + 1 if ending_index < len(current) - 1 else -1

        # check top
        top_got = any([char for char in previous[bounded_left: bounded_right] if not (char.isdigit() or char == '.')])
        # check left
        left_got = not(current[bounded_left].isdigit() or current[bounded_left] == '.')

        # check right
        right_got = not(current[bounded_right - 1].isdigit() or current[bounded_right - 1] == '.')

        # check bottom
        bottom_got = any([char for char in next[bounded_left: bounded_right] if not (char.isdigit() or char == '.')])

        if top_got or left_got or right_got or bottom_got:
            parts.append(number)
    print(parts)
    return parts

final_list = []

for index, row in enumerate(puzzle):

    # index = 0
    # row = puzzle[index]
    try:
        next_row = puzzle[index + 1]
    except:
        next_row = "." * len(row)

    if index == 0:
        prev_row = "." * len(row)
    else:
        prev_row = puzzle[index - 1]

    for item in check_line(prev_row, row, next_row):
        final_list.append(item)
    

print(sum(list(map(int, final_list))))
