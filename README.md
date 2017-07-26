# hlangc
Simple `monkey-lang 2 assembly` compiler
Please notice that only gcc *.s format is supported
If you want to compile a program,you must have gcc installed first...
_compiling with just GNU as from 100%monkey-lang code to shared object is not possible_

## compile
```shell
git clone https://git.coding.net/duangsuse/hlangc.git
cd hlangc &&cargo build
```

## compile a file
use `cargo install` to install it on your system,then
```shell
hlangc c foo
```
command-line usage:
hlangc dump|__d__ [file] -> `dump AST tree`
hlangc compile|__c__ [file] -> `compile file`
hlangc run|__r__ [file] -> `compile file and run compile output`
hlangc wrap|__w__ [file] -> `just translate to assembly`
hlangc help|__h__ -> `print help`
hlangc version|__v__ -> `print version`

## Licence
__GPLv3__