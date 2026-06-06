import { mount } from 'svelte';

// Phosphor icon weights (MIT). "light" gives the flowy, thin green-tech feel.
import '@phosphor-icons/web/light';
import '@phosphor-icons/web/regular';
import '@phosphor-icons/web/fill';

import './app.css';
import App from './App.svelte';

const app = mount(App, {
  target: document.getElementById('app')!,
});

export default app;
