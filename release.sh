# Release all code directories in the repository using cargo cargo build --release.
for DIR in */; do
    DIRNAME=$(basename "$DIR")
    (echo "==> $DIRNAME <==")
    (cd $DIR && (cargo build --release || echo "Build failed."))
done

echo "Releasing complete."