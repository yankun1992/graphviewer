/*
 * Copyright 2022 Yan Kun
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { createApp } from "vue";
import Antd from "ant-design-vue";
import App from "./App.vue";
import "ant-design-vue/dist/antd.css";
import router from "./router";
import "./css/common.css";
import store from "./store";
import { invoke } from "@tauri-apps/api";
import { AppConfig } from "./lib/setting";

const app = createApp(App);
app.use(router);
app.use(Antd);
app.use(store);

Promise.all([invoke("is_first_running"), invoke("read_app_config"), invoke("read_history")]).then((response) => {
    console.log(response);
    store.commit("is_first_running", <boolean>response[0]);
    store.commit("set_config", <AppConfig>response[1]);
    store.commit("set_histories", <History[]>response[2]);
    app.mount("#app");
});

