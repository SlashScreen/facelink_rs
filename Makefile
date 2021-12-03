release:
	if not exist releases\release\src mkdir releases\release
	cargo build --release
	wsl cp -v target/release/facelink_rs.exe releases/release
	wsl cp -v -r releasepackage/* releases/release
	wsl cp -v src/messages.json releases/release/src