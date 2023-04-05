# Test all code directories in the repository using cargo test.
for DIR in */; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo test --quiet)
done

echo "Testing complete."