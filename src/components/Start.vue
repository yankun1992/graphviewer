<template>

    <a-result status="success" title="Create or Open" sub-title="Create a new project or open a exists project!">
        <template #icon>
            <h1>GraphViewer</h1>
        </template>
        <template #extra>
            <a-button key="console" type="primary" @click="newProject()">New Project</a-button>
            <a-button key="buy" @click="openProject()">Open Project</a-button>
        </template>
    </a-result>

    <suspense>
        <Recent></Recent>
    </suspense>

    <Config></Config>

    <suspense>
        <Create :visible="create" @cancelCreate="cancelCreate()"></Create>
    </suspense>

</template>

<script setup lang="ts">
import { onBeforeMount, onMounted, reactive, ref } from 'vue';
import { useRouter } from "vue-router";
import { CREATE } from "../router/name";
import { invoke } from '@tauri-apps/api';
import { selectDir } from '../lib/utils';
import { useStore } from 'vuex';
import Config from "../components/Config.vue"
import Recent from "../components/Recent.vue"
import Create from "../components/Create.vue"
import { TABLE, WORKSPACE } from '../router/name';
import { dirname, basename } from '@tauri-apps/api/path';
import { ProjectInfo } from '../lib/common';


const store = useStore();
const router = useRouter();

const create = ref(false);

// new project
function newProject() {
    create.value = true;
};

function cancelCreate() {
    create.value = false;
}

// open project
async function openProject() {
    const path = await selectDir();
    if (path !== null) {
        const name = await basename(path)
        const info = <ProjectInfo>{ name: name, path: path }
        store.commit('set_opened', info);
        console.log(info);
        router.push(WORKSPACE);
    };
};

</script>

<style scoped>
</style>