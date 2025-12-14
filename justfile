doc *FLAGS:
    cargo doc -p rustorio -p rustorio-engine --no-deps {{ FLAGS }}

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