release:
	if not exist releases\release mkdir releases\release
	cargo build --release
	wsl ls target/release
	wsl cp -v target/release/facelink_rs.exe releases/release
	wsl cp -v -r releasepackage/* releases/release
	wsl cp -v src/messages.json releases/release