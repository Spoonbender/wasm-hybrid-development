@echo off

SET wasm_file_name=demo_app.wasm
SET release_file_path=target/wasm32-unknown-unknown/release/%wasm_file_name%
SET filename2=www/wasm/demo_app_bg.wasm

echo build for WASM32
cargo build --target wasm32-unknown-unknown --release
REM Print file size
for %%A in (%release_file_path%) do echo.Size of "%%A" is %%~zA bytes

echo Run GC to make the WASM much smaller
wasm-gc %release_file_path%
REM Print file size
for %%A in (%release_file_path%) do echo.Size of "%%A" is %%~zA bytes

echo Run wasm-opt to make it even smaller!
call wasm-opt -Os %release_file_path% -o %release_file_path%

REM Print file size
for %%A in (%release_file_path%) do echo.Size of "%%A" is %%~zA bytes

echo generate bindings for JS
wasm-bindgen %release_file_path% --out-dir ./www/wasm/

REM Print file size
for %%A in (%filename2%) do echo.Size of "%%A" is %%~zA bytes

echo copy Rust demos
xcopy src\*.rs www\static\ /Y >NUL

echo copy JavaScript demos
xcopy www\demos\*.* www\static\ /Y >NUL
xcopy www\wasm\demo_app.d.ts www\static\ /Y >NUL

pushd www
call tsc demos\typescripting.ts >NUL
call npm run build
popd