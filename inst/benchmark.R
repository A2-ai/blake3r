res <- bench::mark(check = FALSE,iterations = 5,
  read_file = hash_file_with_blake3_direct_impl("/Users/devin/repos/a2-ai/test-dvs/test1/test_data.csv"),
  mmap = hash_file_with_blake3_impl("/Users/devin/repos/a2-ai/test-dvs/test1/test_data.csv"),
  digest = digest::digest(file = "/Users/devin/repos/a2-ai/test-dvs/test1/test_data.csv")
)
res |> select(expression, median) |>
  arrange(median) |>
  mutate(relative_speedup = as.numeric(median/min(median)))
