# tree-sitter-typescript Wasm Repro Repo

The repo demonstrates broken Wasm support in tree-sitter-typescript.

It is using the forks:

- [tree-sitter](https://github.com/kossnocorp/tree-sitter/tree/wasm-all)
- [tree-sitter-typescript](https://github.com/kossnocorp/tree-sitter-typescript/tree/wasm)

The fixed version is in the `fix` branch.

## Reproduction Steps

To see the problem, install [wasm-pack](https://drager.github.io/wasm-pack/installer/) and run:

```bash
wasm-pack test --node
```

You should see an error about missing `stdlib.h`.

## Details

The fixes are based on the tree-sitter-ruby fixes and follow the same approach. See:

- [tree-sitter PR](https://github.com/tree-sitter/tree-sitter/pull/5158)
- [tree-sitter-ruby PR](https://github.com/kossnocorp/tree-sitter-php/pull/1)
- [Repro Repo](https://github.com/kossnocorp/tree-sitter-ruby-wasm-repro)
