from traceback import print_tb


choices = \
    {
        5: {},
        4: {
            1: {}
        },
        3: {
            1: {
                1: {}
            },
            2: {}
        },
        2: {
            3: {},
            2: {
                1: {}
            },
            1: {
                2: {},
                1: {
                    1: {}
                }
            }
        },
        1: {
            4: {},
            3: {
                1: {}
            },
            2: {
                2: {},
                1: {
                    1: {}
                }
            },
            1: {
                3: {},
                2: {
                    1: {}
                },
                1: {
                    2: {},
                    1: {
                        1: {}
                    }
                }
            }
        }
    }


def case1():
    return [1, 1, 2, 1, 1, 5]
    #       -  -  -  -

def case2():
    return [3, 1, 2, 5, 5]
    #       -     -

def case3():
    return [4, 4, 1, 4]
    #       -     -

def take_path(tree, path):
    return take_path(tree[path[0]], path[1:]) if path else tree


def _pick_out(result, queue):
    if len(queue) == 0:
        return
    tree_path = result
    tree = take_path(choices, tree_path)
    n = queue[0]
    subtree = tree.get(n, None)
    if subtree is None:
        # print('B1')
        # print(f'queue {queue}, result {result}, tree_path {tree_path}')
        return _pick_out(result[:-1], queue)
    else:
        _result = [*result, n]

        if subtree == {}:
            # print('B2')
            return _result
        else:
            # print('B3')
            return _pick_out(_result, queue[1:])


def pick_out(case):
    return _pick_out([], case)

assert pick_out(case1()) == [1, 1, 2, 1]
assert pick_out(case2()) == [3, 2]
assert pick_out(case3()) == [4, 1]
assert pick_out([1, 1, 3, 2, 4]) == [1, 1, 3]
assert pick_out([1, 3, 2, 4]) == [1, 4]
assert pick_out([4, 4]) == None

# print(pick_out([4, 2, 2, 1]))
# print(pick_out([4, 2, 1]))


def build_tree(n):
    return {i: build_tree(n - i) for i in range(1, n + 1)} if n != 0 else {}

assert build_tree(5) == choices



from pprint import pprint
pprint(build_tree(5))

