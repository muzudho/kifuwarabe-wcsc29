# kifuwarabe-wcsc29

ディレクトリ構成を合わせろ。設定ファイルを書け。

いっぱつコンパイルセット☆（＾～＾）

```Shell
cls
set RUST_BACKTRACE=1
cd C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29-lib
cargo update
cargo clippy --example main
cargo build --release
cd C:/muzudho/projects_rust/kifuwarabe-wcsc29
cargo update
cargo clippy
cargo build --release
```

```Shell
### Run.
cargo run --release
```
 
Execution file.
C:/muzudho/projects_rust/rust-kifuwarabe-wcsc29/target/release/rust-kifuwarabe-wcsc29.exe

## How to convert .kif record?

```Shell
cd C:/muzudho/projects_rust/kifuwarabe-wcsc29/target/release
cls
 
### Run.
kifuwarabe-wcsc29 --input "C:/shogi-record/go/eating/wcsc/１回戦/kifu.csa" --output "C:/muzudho/shogi-record/rpm/wcsc/１回戦/kifu"

kifuwarabe-wcsc29 --input "C:/shogi-record/formation-go/wcsc28_kifu/WCSC_F1_APR_MCB.csa" --output "C:/muzudho/shogi-record/rpm-manually/WCSC_F1_APR_MCB.csa"

kifuwarabe-wcsc29 --input "C:/shogi-record/eating-go/formation-go$%wcsc28_kifu$%WCSC28_F4_QHA_APR.csa" --output "C:/muzudho/shogi-record/tape-fragments/formation-go$%wcsc28_kifu$%WCSC28_F4_QHA_APR.tapesfrag"

kifuwarabe-wcsc29 --input "C:/shogi-record/eating-went/formation-go$%wcsc28_kifu$%WCSC28_F4_HNW_PAL.csa" --output "C:/shogi-record/eating-went/formation-go$%wcsc28_kifu$%WCSC28_F4_HNW_PAL.tapesfrag"

kifuwarabe-wcsc29 --input "C:/muzudho/wcsc29-master/shogi-record/eating-go/formation-go$%wcsc28_kifu$%WCSC28_F4_DGK_TNK.csa" --output "C:/muzudho/wcsc29-master/shogi-record/tapes-fragments/formation-go$%wcsc28_kifu$%WCSC28_F4_DGK_TNK"

# from error folder.
kifuwarabe-wcsc29 --input "C:/muzudho/wcsc29-master/shogi-record/converter-error/wcsc28_kifu$%WCSC_F3_TNK_MCB_2[SJ-U8].csa" --output "C:/muzudho/wcsc29-master/shogi-record/tapes-fragments/wcsc28_kifu$%WCSC_F3_TNK_MCB_2[SJ-U8]"

echo %ERRORLEVEL%
```
