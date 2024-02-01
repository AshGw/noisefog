export RUST_BACKTRACE := "1"
alias s:= setup 
alias h:= set-hooks
@setup:
    # rustup install nightly
    rustup component add clippy-preview
    pip install pre-commit
    cargo build

@build:
    cargo install wasm-pack
    wasm-pack build --target bundler
    npm i
    npm run build

@serve:
    npm run serve

@clean:
    rm -rf Cargo.lock pkg target node_modules dist package-lock.json
    
@set-hooks:
    pre-commit
    bash ./hooks/pre-push

