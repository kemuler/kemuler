cargo +nightly doc --no-deps --target-dir target --features "spin_sleep"
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=kemuler\">" > target/doc/index.html
cp -r target/doc ./docs
