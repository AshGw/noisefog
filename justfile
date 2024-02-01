export RUST_BACKTRACE := "1"
alias s:= setup 
alias h:= set-hooks
@setup:
    # rustup install nightly
    rustup component add clippy-preview
    pip install pre-commit
    cargo build


@set-hooks:
    pre-commit
    bash ./hooks/pre-push

