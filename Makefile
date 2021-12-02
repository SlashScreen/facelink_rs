release:
	if not exist release mkdir release
	cargo build --release
	wsl ls 
	$(shell ls)
	$(shell cp -v target/release/facelink_rs.exe release)
	$(shell cp -v releasepackage/* release)
	$(shell cp -v src/messages.json release)