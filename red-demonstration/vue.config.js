module.exports = {
  chainWebpack: (config) => {
    config.entry("app").clear();
    config.entry("app").add("./src/bootstrap.js");
  },

  css: {
    loaderOptions: {
      sass: {
        additionalData: '@import "@/scss/variables.scss";',
      },
    },
  },
  devServer: {
    proxy: {
      "^/reds": {
        target: "http://localhost:8000",
        changeOrigin: true,
        secure: false,
        logLevel: "debug",
      },
    },
  },
};
