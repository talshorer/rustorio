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

publish:
    cargo publish --allow-dirty