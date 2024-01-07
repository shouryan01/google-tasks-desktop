import App from "./App.svelte";

const targetElement = document.getElementById("app");
const app = new App({
  target: targetElement ? targetElement : document.body,
});

export default app;
