<template>
    <a-modal v-model:visible="visible" title="Setting" @ok="handleOk" okText="Save">
        <a-form ref="formRef" :model="store.state.appConfig" layout="vertical" name="form_in_modal">
            <a-form-item label="Project Home">
                <a-input @click="change_projects_home()" v-model:value="store.state.appConfig.projects_home" />
            </a-form-item>
            <a-form-item label="History File">
                <a-input v-model:value="store.state.appConfig.history_file" :disabled="true" />
            </a-form-item>
        </a-form>
    </a-modal>
</template>

<script setup lang="ts">
import { onBeforeMount, onMounted, ref } from 'vue';

import { useStore } from 'vuex';
import { selectDir } from '../lib/utils';
import { invoke } from '@tauri-apps/api';
import type { FormInstance } from 'ant-design-vue';

const store = useStore();

// setting
const visible = ref<boolean>(false);

const formRef = ref<FormInstance>();



const showModal = () => {
    visible.value = true;
};

const handleOk = (e: MouseEvent) => {
    console.log(e);
    visible.value = false;
};

async function change_projects_home() {
    const path = await selectDir();
    if (path === null) {
        const default_home: string = await invoke('get_default_projects_home');
        store.commit('set_projects_home', default_home);
    } else {
        store.commit('set_projects_home', path);
    };
};

onBeforeMount(async () => {
    const is_first_running = <boolean>store.state.isFirstRunning;
    console.log("onMounted is_first_running: " + is_first_running);
    if (is_first_running) {
        showModal();
    };
    // setTimeout(() => {
    // }, 1000);
});

onMounted(async () => {

});

</script>

<style>
.collection-create-form_last-form-item {
    margin-bottom: 0;
}
</style>