from array import array
from collections import deque
from copy import copy
from dataclasses import dataclass
from typing import Deque, Iterable, List, NamedTuple, Optional

N = 3

class Point(NamedTuple):
    x: int
    y: int

    @property
    def index(self):
        return self.x * N + self.y

    def is_within_bounds(self):
        return 0 <= self.x < N and 0 <= self.y < N

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

@dataclass
class Node:
    zero: Point
    parent: Optional["Node"]
    board: Optional[array] = None

    def __iter__(self):
        current = self
        while current:
            yield current
            current = current.parent

MOVES: List[Point] = [Point(0, 1), Point(0, -1), Point(1, 0), Point(-1, 0)]

def solver(root: Node, completed: Node) -> Node:
    queue: Deque[Node] = deque()

    queue.appendleft(root)

    while len(queue) > 0:
        current = queue.popleft()

        if current.zero == completed.zero:
            board = gen_board(root, current)

            if board == completed.board:
                return current

            del board

        parent_zero = current.parent.zero if current.parent else Point(-1, -1)

        for m in gen_new_moves(current.zero, parent_zero):
            new_node = Node(
                zero=m,
                parent=current
            )
            queue.append(new_node)

def gen_new_moves(current_zero: Point, parent_zero: Point) -> Iterable[Point]:
    return filter(
        lambda p: p.is_within_bounds() and p != parent_zero,
        map(
            lambda m: m + current_zero,
            MOVES
        )
    )

def print_board(board):
    for (i, x) in enumerate(board):
        print(x, end='\n' if i%N == N-1 else ' ')
    print()

def gen_board(root: Node, node: Node):
    board = copy(root.board)

    for move in reversed(list(node)):
        if move.parent:
            value = board[move.zero.index]
            board[move.parent.zero.index] = value
            board[move.zero.index] = 0

    return board

def main():
    root = Node(
        board=array('i', [
            7, 8, 4,
            5, 1, 6,
            0, 3, 2
        ]),
        zero=Point(2, 0),
        parent=None
    )

    completed = Node(
        board=array('i', [
            1, 2, 3,
            4, 5, 6,
            7, 8, 0
        ]),
        zero=Point(2, 2),
        parent=None
    )

    print(solver(root, completed))

if __name__ == "__main__":
    main()