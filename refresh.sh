#!/bin/bash

BINARY_NAME="damn"
SOURCE_PATH="target/release/$BINARY_NAME"
INSTALL_DIR="${HOME}/.local/bin"
DEST_PATH="${INSTALL_DIR}/${BINARY_NAME}-bin"

cargo build --release || { echo "Error: Cargo build failed." >&2; exit 1; }
mkdir -p "${INSTALL_DIR}" || { echo "Error: Failed to create directory ${INSTALL_DIR}." >&2; exit 1; }
cp "${SOURCE_PATH}" "${DEST_PATH}" || { echo "Error: Failed to copy binary from ${SOURCE_PATH} to ${DEST_PATH}." >&2; exit 1; }

echo "Project refreshed and binary installed to ${DEST_PATH}"