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

import Start from "../components/Start.vue";
import Create from "../components/Create.vue";
import MainPage from "../components/workspace/MainPage.vue";
import { createRouter, createWebHashHistory } from "vue-router";
import { CREATE, ROOT, START, WORKSPACE } from "./name";

const routes = [
    { path: "/start", name: START, component: Start },
    { path: "/", name: ROOT, component: Start },
    { path: "/create", name: CREATE, component: Create },
    {
        path: "/workspace", name: WORKSPACE, component: MainPage
    }
];


const router = createRouter({
    history: createWebHashHistory(),
    routes
});

export default router;