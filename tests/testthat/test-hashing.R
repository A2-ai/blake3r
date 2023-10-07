# data_paths environment is provided from the setup utility
test_that("blake3 hashing works", {
  # tested against b3sum reference utility for consistent hash
  expect_equal(hash_file_with_blake3(data_paths$data1),
               "5adefccbc8ee78dbba74072aac9f60aaa0c0f68c585dc6b5e6d072d062b96818"
  )
})

test_that("a nonexistent file will error with a clear error message that repeats the path", {
  expect_error(hash_file_with_blake3("path/to/filethatdoesntexist.csv"),
               "file does not exist at: path/to/filethatdoesntexist.csv")
})
