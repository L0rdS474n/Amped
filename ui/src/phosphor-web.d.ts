// @phosphor-icons/web exposes each icon weight as a CSS side-effect entry point
// with no bundled type declarations. Declare them so svelte-check / tsc accept the
// `import '@phosphor-icons/web/<weight>'` statements in main.ts.
declare module '@phosphor-icons/web/thin';
declare module '@phosphor-icons/web/light';
declare module '@phosphor-icons/web/regular';
declare module '@phosphor-icons/web/bold';
declare module '@phosphor-icons/web/fill';
declare module '@phosphor-icons/web/duotone';
