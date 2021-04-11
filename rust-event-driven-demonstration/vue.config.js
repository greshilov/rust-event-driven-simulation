module.exports = {
  chainWebpack: (config) => {
    config.entry("app").clear();
    config.entry("app").add("./src/bootstrap.js");
  },
  devServer: {
    proxy: {
      '^/api': {
        target: 'http://localhost:8000',
        changeOrigin: true,
        secure: false,
        logLevel: "debug"
      },
    }
  },
};
