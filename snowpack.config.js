module.exports = {
    mount: {
        "js": "/js",
        "static": "/"
    },
    devOptions: {
        open: "none"
    },
    plugins: [
        ['@emily-curry/snowpack-plugin-wasm-pack', {
            projectPath: '.'
        }]
    ]
}