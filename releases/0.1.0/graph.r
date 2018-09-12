#!/usr/bin/env Rscript
args = commandArgs(trailingOnly=TRUE)

# test if there is at least one argument: if not, return an error
if (length(args)==0) {
  stop("At least one argument must be supplied!", call.=FALSE)
}

l = sapply(strsplit(args[1], ','), as.numeric)

png(file = args[3])
plot(l, type = "l", lwd=5, xlab="Time", ylab="Price", main=args[2])
dev.off()
