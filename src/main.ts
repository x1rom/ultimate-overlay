import { createApp } from "vue";
import App from "./App.vue";

import { createVuetify } from "vuetify";
import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

const app = createApp(App);

app.use(
    createVuetify({
        components,
        directives,
    })
);

app.mount("#app");
