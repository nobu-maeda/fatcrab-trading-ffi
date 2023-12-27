NAME="FatCrabTrading"
APP_NAME="OceanSea"
HEADERPATH="bindings/${NAME}FFI.h"
TARGETDIR="target"
OUTDIR="../${APP_NAME}/${APP_NAME}/"
RELDIR="release"
STATIC_LIB_NAME="libfatcrab_trading.a"
NEW_HEADER_DIR="bindings/include"

cargo build --target aarch64-apple-darwin --release
cargo build --target aarch64-apple-ios --release
cargo build --target aarch64-apple-ios-sim --release

# UniFfi bindgen
cargo run --bin uniffi-bindgen generate "src/fatcrab_trading.udl" --language swift --out-dir ./bindings

mkdir -p "${NEW_HEADER_DIR}"
cp "${HEADERPATH}" "${NEW_HEADER_DIR}/"
cp "bindings/${NAME}FFI.modulemap" "${NEW_HEADER_DIR}/module.modulemap"

rm -rf "${OUTDIR}/${NAME}.swift"
cp "bindings/${NAME}.swift" "${OUTDIR}/${NAME}.swift"

rm -rf "${OUTDIR}/${NAME}_framework.xcframework"
xcodebuild -create-xcframework \
    -library "${TARGETDIR}/aarch64-apple-darwin/${RELDIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/aarch64-apple-ios/${RELDIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -library "${TARGETDIR}/aarch64-apple-ios-sim/${RELDIR}/${STATIC_LIB_NAME}" \
    -headers "${NEW_HEADER_DIR}" \
    -output "${OUTDIR}/${NAME}_framework.xcframework"