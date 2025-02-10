#!/bin/sh

echo helo wrld | ./rs-find-prefix-ascii helo
echo helo wrld | ./rs-find-prefix-ascii wrld
echo wrld helo | ./rs-find-prefix-ascii wrld
echo wrld helo | ./rs-find-prefix-ascii helo
