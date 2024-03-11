defualt:
    @just -l

loc:
    find . -name "*.rs" | xargs cat | wc -l

run:
    cargo r --features log
