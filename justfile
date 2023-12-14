set shell := ["nu", "-c"]

run: 
  cargo watch -x "shuttle run"
  
deploy:
  cargo shuttle deploy
  
test day="--all":
  cch23-validator {{day}}
  
ngrok:
  ngrok http 8000