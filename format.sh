# Format all code directories in the repository using cargo fmt.
for DIR in */; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo fmt --quiet)
done

echo "Formatting complete."