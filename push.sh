#!/bin/bash
if [ -z "$1" ]; then
    MSG="update core logic"
else
    MSG="$1"
fi

git add .
git commit -m "$MSG"
git push
echo "--------------------------------------------------------"
echo "--> Push selesai! GitHub Actions sedang mengompilasi .so"
echo "--------------------------------------------------------"
