# 2025 Steven Chiacchira
# seems this is way too slow :(
# probably because matrix is only 16x16, and torch is not meant for this
from typing import Final, List

import torch
from torch import Tensor
from torch.nn import Conv2d, Module, Parameter


# because we are massively abusing PyTorch, we do not need to enable many of its features
torch.set_grad_enabled(False)
torch.set_default_device("cuda" if torch.cuda.is_available() else "cpu")


class AutomataIter(Module):
    """Implementation of our Key Automata rule using PyTorch. Allows fast encryption via the GPU."""

    moore_kernel: Final[List[List[int]]] = [[1, 1, 1], [1, 10, 1], [1, 1, 1]]

    neumann_kernel: Final[List[List[int]]] = [[0, 1, 0], [1, 10, 1], [0, 1, 0]]

    def __init__(self, channels: int) -> None:
        super().__init__()
        self._conv_layer = Conv2d(
            channels,
            channels,
            (3, 3),
            stride=(1, 1),
            bias=False,
            padding=1,
            padding_mode="circular",
        )
        self._conv_layer.weight = Parameter(
            torch.tensor(
                [[AutomataIter.moore_kernel] * channels] * channels,
                dtype=torch.float16,
            ),
            requires_grad=False,
        )

    @staticmethod
    def set_cell_status(score: Tensor) -> Tensor:
        # for torch to cooperate we need to build a function which maps these
        # values:
        # 0, 1, 7, 8, C, C+1, C+5, C+6, C+7, C+8 -> 0
        # 2, 3, 4, 5, 6, C+2, C+3, C+4, C+5 -> 1
        # where C ~\in [-8, 8] is an offset parameter which is the center of
        # `moore_kernel`. Because we're working with float values,
        # `torch.clamp()` seems promising. We can find an expression `g` which
        # is positive for values to be sent to 1 and negative otherwise. Then
        # we just compute clamp(inf * g, 0, 1) to get our function.
        # If we plot the points we want on a Cartesian space, we see that we
        # really just need a polynomial that crosses the x axis at
        # x=1.5, 6.5, C+1.5, and C+4.5. So we choose the factors
        # (x-1.5)(x-6.5)(x-C-1.5)(x-C-4.5). We notice the polynomial is
        # improperly oriented and multiply by -1. This yields the expanded form
        # We might reasonably worry about floating point error, but really both
        # x and our coefficients are sums of powers of 2, so there should be no
        # problems as long as C is also a power of 2. We choose C=10.
        # The polynomial also is quite large, so multiplying by `inf` is
        # ultimately unnecessary.
        # Bit shifting and/or a lookup table may be faster
        scaled = -(score - 1.5) * (score - 6.5) * (score - 11.5) * (score - 14.5)
        return torch.clamp(scaled, 0, 1)

    def forward(self, inputs: Tensor) -> Tensor:
        scores = self._conv_layer(inputs)
        return AutomataIter.set_cell_status(scores)


class AutomataBlock(Module):
    def __init__(self, iters: int, channels: int = 2) -> None:
        super().__init__()
        self._iters = iters
        self._automata = AutomataIter(channels)

    def forward(self, inputs: Tensor) -> Tensor:
        intermediate = inputs
        for _ in range(self._iters):
            intermediate = self._automata(intermediate)

        return intermediate


next_state: Tensor = torch.rand((1, 2, 16, 16), dtype=torch.float16).round()

print(next_state)
print(next_state)

automata_block_gen = torch.jit.script(AutomataBlock(3_000_000, 2))
with torch.no_grad():
    for _ in range(5):
        next_state = automata_block_gen(next_state)
        print(next_state.sum())
        print(next_state)
