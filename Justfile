run day:
	cargo run --package d"$(printf "%02d" "{{ day }}")"

test day:
	cargo test --package d"$(printf "%02d" "{{ day }}")"

prepare day:
	./manage.sh {{ day }}

# vim: ft=make
