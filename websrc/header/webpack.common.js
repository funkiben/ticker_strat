module.exports = {
    entry: {
        header: './src/header.js',
    },
    output: {
        filename: '[name].js',
    },
    module: {
        rules: [
            {
                // loading css
                test: /\.css$/i,
                loader: 'css-loader'
            },
            {
                // loading html
                test: /\.html$/i,
                loader: 'html-loader'
            },
            {
                // loading fonts
                test: /\.(png|svg|jpg|gif|woff|woff2|eot|ttf|otf)$/i,
                use: [
                    'file-loader'
                ]
            }
        ]
    }
}