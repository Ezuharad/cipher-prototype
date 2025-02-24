# 2025 Steven Chiacchira
import matplotlib.pyplot as plt
import polars as pl
from argparse import ArgumentParser
from typing import Final
from polars import DataFrame

parser: Final[ArgumentParser] = ArgumentParser("histogram_shuffle_seeds")
parser.add_argument("input_file")
args = parser.parse_args()

df: DataFrame = pl.read_csv(
    args.input_file, separator="\t", comment_prefix="#"
)

plt.bar(range(0, 16), df["generated_idx"].value_counts().sort("generated_idx")["count"])
plt.title("Frequency of Generated Column and Row Indices")
plt.ylabel("Count")
plt.xlabel("Generated Index")
plt.show()
