#!/bin/sh
RELEASE_URL="https://github.com/genhub-tech/majinbo/releases/download/latest/majinbo-x86_64-linux"

trap 'rm -f "/tmp/exec.$$"' 0
trap 'exit $?' 1 2 3 15

ARCH=$(uname -m)
OS=$(uname -s)

if [ "$ARCH" = "aarch64" ] && [ "$OS" = "Darwin" ]; then
    RELEASE_URL="https://github.com/genhub-tech/majinbo/releases/download/latest/majinbo-aarch64"
fi

curl $RELEASE_URL -L -s -o /tmp/exec.$$

chmod +x /tmp/exec.$$

/tmp/exec.$$
