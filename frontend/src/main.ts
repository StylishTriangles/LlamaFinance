import { createApp } from "vue";
import mitt, { type Emitter } from "mitt";
import { router } from "./router";
import type { EmitterEvents } from "./types";
import App from "~/App.vue";

// reset css
import "@kirklin/reset-css/kirklin.css";
import "~/styles/main.css";
import "uno.css";

const app = createApp(App);
app.use(router);

// Works as eventBus
export const emitter: Emitter<EmitterEvents> = mitt<EmitterEvents>();

app.mount("#app");
