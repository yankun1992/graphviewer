
import Start from '../components/Start.vue'
import Create from '../components/Create.vue'
import MainPage from '../components/workspace/MainPage.vue'
import Table from '../components/workspace/Table.vue'
import { createRouter, createWebHashHistory } from "vue-router"
import { CREATE, ROOT, START, TABLE, WORKSPACE } from './name';

const routes = [
    { path: '/start', name: START, component: Start },
    { path: '/', name: ROOT, component: Start },
    { path: '/create', name: CREATE, component: Create },
    {
        path: '/workspace', name: WORKSPACE, component: MainPage
    }
];


const router = createRouter({
    history: createWebHashHistory(),
    routes
})

export default router;