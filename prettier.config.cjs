module.exports = {
  printWidth: 120,
  tabWidth: 2,
  trailingComma: 'es5',
  arrowParens: 'always',
  singleQuote: true,
  semi: true,
  plugins: ['prettier-plugin-astro', '@trivago/prettier-plugin-sort-imports'],
  importOrder: ['^vitest', '<THIRD_PARTY_MODULES>', '^@(.*)$', '^[.]/', '^[.]{2,}/'],
  importOrderSeparation: true,
  importOrderSortSpecifiers: true,
  overrides: [
    {
      files: '*.astro',
      options: {
        parser: 'astro',
      },
    },
  ],
};
