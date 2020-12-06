from functools import reduce

def all_answer(para):
    return len(reduce(set.intersection,map(set, str.splitlines(para))))

print(sum(map(all_answer,open('input.txt').read().split("\n\n"))))
