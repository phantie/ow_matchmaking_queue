
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

def try_take_path(tree, path):
    try:
        return take_path(tree, path)
    except KeyError:
        return None


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


def build_tree(n):
    return {i: build_tree(n - i) for i in range(1, n + 1)} if n != 0 else {}


# Best algorithm
def _pick_out(tree_path, queue):
    tree = choices

    for i, l in enumerate(queue):
        subtree_path = [*tree_path, l]
        subtree = try_take_path(tree, subtree_path)
        if subtree is None:
            continue
        elif subtree == {}:
            return subtree_path
        else:
            if (r := _pick_out(subtree_path, queue[i + 1:])) is not None:
                return r

def pick_out(case):
    return _pick_out([], case)


def resolved_path(tree_nesting, path):
    total = tree_nesting
    for path_node in path:
        assert path_node <= tree_nesting
        if total - path_node >= 0:
            total -= path_node
        else: # total - path_node < 0
            return False
    return True


# cleanest
def resolved_path(tree_nesting, path):
    assert all(1 <= node <= tree_nesting for node in path)
    assert sum(path) >= 0
    return sum(path) <= tree_nesting


assert resolved_path(5, [5])
assert resolved_path(5, [1, 1, 1, 1, 1])
assert resolved_path(5, [1, 1, 1])
assert not resolved_path(5, [3, 3])


# Best algorithm with no required precalculated tree
def _pick_out(tree_path, queue):
    tree_nesting = 5

    for i, l in enumerate(queue):
        subtree_path = [*tree_path, l]
        subtree = resolved_path(tree_nesting, subtree_path)

        if not subtree:
            continue
        elif sum(subtree_path) == tree_nesting:
            return subtree_path
        else:
            if (r := _pick_out(subtree_path, queue[i + 1:])) is not None:
                return r


def pick_out(case):
    return _pick_out([], case)

assert pick_out(case1()) == [1, 1, 2, 1]
assert pick_out(case2()) == [3, 2]
assert pick_out(case3()) == [4, 1]
assert pick_out([1, 1, 3, 2, 4]) == [1, 1, 3]
assert pick_out([1, 3, 2, 4]) == [1, 4]
assert pick_out([4, 4]) == None

assert pick_out([4, 2, 2, 1]) == [4, 1]
assert pick_out([3, 4, 2, 1]) == [3, 2]
assert pick_out([4, 2, 1]) == [4, 1]
assert pick_out([5]) == [5]
assert pick_out([4, 5, 4]) == [5]
assert pick_out([4, 5, 4, 1]) == [4, 1]
assert pick_out([3, 4, 5, 4, 1]) == [4, 1]
assert pick_out([3, 3, 4, 5, 4, 1]) == [4, 1]

assert build_tree(5) == choices



from pprint import pprint
# pprint(build_tree(5))

