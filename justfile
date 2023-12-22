set shell := ["nu", "-c"]

start:
  hx src/router.rs

run:
  cargo shuttle run

watch: 
  cargo watch -x "shuttle run" -d 5 -w 'src'
  
deploy:
  cargo shuttle deploy
  
test day="--all":
  cch23-validator {{day}}
  
ngrok:
  ngrok http 8000
  
update:
  cargo install cch23-validator
  
sqlx:
  cargo sqlx prepare
  
check:
  cargo clippy
  
fix:
  cargo clippy --fix --allow-dirty