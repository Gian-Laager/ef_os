nmap <leader><leader>b :!cargo bootimage<CR>
nmap <leader><leader>B :!cargo --release bootimage<CR>

nmap <leader><leader>r :!cargo run<CR>
nmap <leader><leader>R :!cargo --release run<CR>

lua << EOF
local nvim_lsp = require'lspconfig'

nvim_lsp.rust_analyzer.setup({
    settings = {
        ["rust-analyzer"] = {
            imports = {
                granularity = {
                    group = "module",
                },
                prefix = "self",
            },
            cargo = {
                buildScripts = {
                    enable = true,
                },
            },
            procMacro = {
                enable = true
            },

            checkOnSave = {
              allTargets = false,
            },

        },
    }
})
EOF
