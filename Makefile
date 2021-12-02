release:
	cargo build --release
	mkdir release
	pwsh -noprofile -command cp target/release/facelink_rs.exe release