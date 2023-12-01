set shell := ["powershell.exe", "-c"]

hello:
  Write-Host "Hello"

work day part:
  cargo watch -x "check -p {{day}}" -s "just test {{part}} -p {{day}}"

test part +FLAGS='-p day-01':
  cargo nextest run {{FLAGS}} {{part}}

run day part:
  cargo run -p {{day}} --bin {{part}}
