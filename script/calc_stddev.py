# 2025 Steven Chiacchira
import polars as pl

df: pl.DataFrame = pl.read_csv("data/experiment/1_seed_count_trial_3.tsv", separator="\t", comment_prefix="#")

print(df["n_alive"].std())
