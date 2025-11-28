play:
    cargo play

doc *FLAGS:
    rm -rf target/doc
    cargo doc -p rustorio --no-deps {{ FLAGS }}
    rm -rf docs/
    mkdir -p docs
    cp -r target/doc/* docs/

push BRANCH:
    jj bookmark move {{BRANCH}} --to=@-
    jj git push

pull:
    jj git fetch
    jj new main

publish:
    cargo publish --allow-dirty

install-local:
    cargo install --path rustorio