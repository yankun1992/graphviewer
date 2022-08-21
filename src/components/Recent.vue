<template>
    <a-divider :dashed="true">Recent</a-divider>
    <a-list :split="false" size="small" item-layout="horizontal" :data-source="histories"
        style="width: 400px;margin: 0 auto;">
        <template #renderItem="{ item }">
            <a-list-item>
                <a-list-item-meta>
                    <template #title>
                        <a @click="openRecent">{{ item.name }}</a>
                    </template>
                    <template #description>
                        {{ item.path }}
                    </template>
                </a-list-item-meta>
            </a-list-item>
        </template>
    </a-list>
</template>

<script setup lang="ts">

import { invoke } from '@tauri-apps/api';
import { onMounted, ref } from 'vue';
import { useStore } from 'vuex';

const store = useStore();
// recent
const histories: History[] = await invoke('read_history');
// console.log(histories);

function openRecent(event: Event) {
    const ele = <HTMLElement>event.target;
    const name = ele.innerText;

    console.log(name)

};

// onMounted(async () => {
//     histories.value = await invoke('read_history');
// });

</script>

<style>
</style>

