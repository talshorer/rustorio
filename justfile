play:
    cargo play

doc *FLAGS: 
    cargo doc -p rustorio --no-deps {{ FLAGS }}
    cp -r target/doc/* docs/

push BRANCH:
    jj bookmark move {{BRANCH}} --to=@-
    jj git push

