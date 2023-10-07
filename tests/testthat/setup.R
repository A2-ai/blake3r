tmp_data <- as.data.frame(expand.grid(col1 = 1:100, col2 = 1:100, letters = letters))
data_paths <- new.env()
data_path <- file.path(tempdir(), "data1.csv")
data_paths$data1 <- data_path
write.csv(tmp_data, data_path, quote = FALSE, row.names = FALSE)
withr::defer(unlink("mtcars.csv"), teardown_env())
