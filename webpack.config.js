const path = require('path');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js',
  },
  plugins: [
    new CleanWebpackPlugin({
      cleanOnceBeforeBuildPatterns: ['dist'],
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: 'assets',
          to: 'assets/',
          filter: (filepath) => !filepath.includes('.txt'),
        },
      ],
    }),
    new HtmlWebpackPlugin({
      title: 'Viktor Kunovski',
      template: 'index.html',
    }),
  ],
  experiments: {
    asyncWebAssembly: true
  }
};
