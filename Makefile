FRONTEND_DIR=src-tauri

.PHONY: install build dev

install:
	npm install

run:
	npm run tauri dev