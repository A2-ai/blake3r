tmp_data <- expand.grid(col1 = 1:100, col2 = 1:100, letters = letters)
data_paths <- new.env()
data_path <- file.path(tempdir(), "data1.csv")
data_paths$data1 <- data_path

# look at ?write.csv
# There is an IETF RFC4180 (https://www.rfc-editor.org/rfc/rfc4180) for CSV files,
#which mandates comma as the separator and CRLF line endings.
#write.csv writes compliant files on Windows: use eol = "\r\n" on other platforms.

# and above it the note: To write a Unix-style file on Windows, use a binary connection e.g. file = file("filename", "wb").


fcon <- file(data_path, "wb")
write.table(tmp_data, file = fcon,sep=",", eol = "\n", quote = FALSE, row.names = FALSE)
close(fcon)
withr::defer(unlink(data_path), teardown_env())
