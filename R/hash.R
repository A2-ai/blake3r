#' hash a file with the blake3 algorithm
#' @param file path to file
#' @param memmap whether to use memory mapping
#' @param parallelize use parallel hash
#' @export
hash_file_with_blake3 <- function(file, memmap = TRUE, parallelize = TRUE) {
  if (!file.exists(file)) {
    stop(sprintf("file does not exist at: %s", file), call. = FALSE)
  }
  if (!memmap) {
    # note files < 16 bytes are not mem-mapped regardless
    return(hash_file_with_blake3_direct_impl(file))
  }
  return(hash_file_with_blake3_impl(file))
}
