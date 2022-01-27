module.exports = {
  root: true,
  extends: ['@elastic/eslint-config-kibana', 'plugin:@elastic/eui/recommended'],
  ignorePatterns: ['server/wasm/**', 'public/wasm/**'],
  rules: {
    '@kbn/eslint/require-license-header': 'off',
  },
};
