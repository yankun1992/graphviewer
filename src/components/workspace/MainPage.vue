<template>
    <a-layout style="min-height: 100vh">
        <a-layout-sider v-model:collapsed="collapsed" collapsible width="260" theme="light" id="sider">
            <div class="logo" />
            <a-menu v-model:selectedKeys="selectedKeys" v-model:expandedKeys="expandedKeys1" theme="light" mode="inline" :inlineIndent="10"
                style="height: 100%;">
                <a-sub-menu key="project">
                    <template #title>
                        <span>
                            <apartment-outlined />
                            <span>Project</span>
                        </span>
                    </template>

                    <a-directory-tree v-model:expandedKeys="expandedKeys" v-model:selectedKeys="selectedKeys2" multiple
                        :tree-data="treeData">
                        <template #icon="{ key, expanded }">
                            <template v-if="['vertices', 'edges'].some((id) => id === key)">
                                <table-outlined />
                            </template>
                            <template v-else-if="key === 'viewer'">
                                <template v-if="expanded">
                                    <folder-open-outlined />
                                </template>
                                <template v-else>
                                    <folder-outlined />
                                </template>
                            </template>
                            <template v-else>
                                <file-image-outlined />
                            </template>
                        </template>
                        <template #title="{ key: treeKey, title }">
                            <a-dropdown :trigger="['contextmenu']">
                                <span v-on:dblclick="ondblclick(treeKey)">{{ title }}</span>
                                <template #overlay>
                                    <a-menu @click="({ key: menuKey }) => onContextMenuClick(treeKey, menuKey)"
                                        style="width: 200px;">
                                        <template v-if="['vertices', 'edges'].some((id) => id === treeKey)">
                                            <EditMenu :menu="tableMenu"></EditMenu>
                                        </template>
                                        <template v-else-if="treeKey === 'viewer'">
                                            <EditMenu :menu="viewerMenu"></EditMenu>
                                        </template>
                                        <template v-else-if="treeKey.startsWith('viewer')">
                                            <EditMenu :menu="graphMenu"></EditMenu>
                                        </template>
                                    </a-menu>
                                </template>
                            </a-dropdown>
                        </template>
                    </a-directory-tree>
                </a-sub-menu>

            </a-menu>
        </a-layout-sider>
        <a-layout>
            <a-layout-content style="width: 100%; height: 100%; background: #fff;">
                <a-tabs v-model:activeKey="activeKey" hide-add type="editable-card" @edit="onEdit">
                    <a-tab-pane v-for="pane in panes" :key="pane.key" :tab="pane.title" :closable="pane.closable">
                        <template v-if="pane.title === 'vertices' || pane.title === 'edges'">
                            <Table :pageType="pane.type" :columns="pane.data['columns']" :data="pane.data['data']">
                            </Table>
                        </template>
                        <template v-else-if="pane.type === PageType.SETTING">
                            <suspense>
                                <GraphSetting></GraphSetting>
                            </suspense>
                        </template>
                    </a-tab-pane>
                </a-tabs>
            </a-layout-content>
        </a-layout>
    </a-layout>

    <a-modal v-model:visible="createViewerVisiable" title="Create new viewer" @ok="onCreateViewer">
        <a-input v-model:value="currentCreateName" placeholder="Please input graph viewer name." />
    </a-modal>
</template>
<script setup lang="ts">
import {
    FolderOutlined,
    FolderOpenOutlined,
    TableOutlined,
    ApartmentOutlined,
    FileImageOutlined
} from '@ant-design/icons-vue';
import { ref } from 'vue';
import { notification, TreeProps } from 'ant-design-vue';
import EditMenu from "./EditMenu.vue"
import { tableMenu, viewerMenu, graphMenu, PageType, openTable } from '../../lib/workspace'
import { useRouter } from "vue-router";

import { useStore } from 'vuex';
import { TableType } from '../../lib/common';
import { invoke } from '@tauri-apps/api';
import { join } from '@tauri-apps/api/path';
import Table from './Table.vue';
import { computed } from '@vue/reactivity';
import GraphSetting from './GraphSetting.vue';

const router = useRouter();
const store = useStore();

const createViewerVisiable = ref(false);
const currentCreateName = ref<string | null>(null);

function onCreateViewer() {
    if (currentCreateName.value === null) {
        notification['error']({
            message: 'Input Error',
            description:
                'viewer name must not be empty!',
        });
    } else {
        panes.value.push({ title: currentCreateName.value + '(setting)', key: currentCreateName.value, closable: true, type: PageType.SETTING, data: {} });
        createViewerVisiable.value = false;
        activeKey.value = currentCreateName.value;
        treeData.value?.at(2)?.children?.push({ title: currentCreateName.value, key: currentCreateName.value, isLeaf: true });
    }
};

const collapsed = ref<boolean>(false);
const selectedKeys = ref<string[]>(['1']);

const expandedKeys = ref<string[]>(['0-0', '0-1']);
const expandedKeys1 = ref<string[]>(['0']);

const selectedKeys2 = ref<string[]>([]);
const treeData = ref<TreeProps['treeData']>([
    {
        title: 'vertices',
        key: 'vertices',
        isLeaf: true,
    },
    {
        title: 'edges',
        key: 'edges',
        isLeaf: true,
    },
    {
        title: 'viewer',
        key: 'viewer',
        children: [
            {
                title: 'cycle',
                key: 'viewer-cycle',
                isLeaf: true,
            }
        ]
    }
]);

interface ViewPane {
    title: string,
    key: string;
    closable?: boolean,
    type: PageType,
    data: object
};

const panes = ref<ViewPane[]>([]);

const paneSet = computed(() => {
    return new Set(panes.value.map(p => p.title));
});

const activeKey = ref<string | null>(null);
const newTabIndex = ref(0);

const remove = (targetKey: string) => {
    let lastIndex = 0;
    panes.value.forEach((pane, i) => {
        if (pane.key === targetKey) {
            lastIndex = i - 1;
        }
    });
    panes.value = panes.value.filter(pane => pane.key !== targetKey);
    if (panes.value.length && activeKey.value === targetKey) {
        if (lastIndex >= 0) {
            activeKey.value = panes.value[lastIndex].key;
        } else {
            activeKey.value = panes.value[0].key;
        }
    }
};

const onEdit = (targetKey: string) => {
    remove(targetKey);
};

function onContextMenuClick(treeKey: string, menuKey: string | number) {
    switch (treeKey) {
        case 'vertices':
            if (menuKey === 'open') { ondblclick(treeKey); };
            break;
        case 'edges':
            if (menuKey === 'open') { ondblclick(treeKey); };
            break;
        case 'viewer':
            if (menuKey === 'create') { createViewerVisiable.value = true };
            break;
        case 'viewer-':

    };
};

async function ondblclick(treeKey: string) {
    if (paneSet.value.has(treeKey)) {
        activeKey.value = treeKey;
    } else {
        const pageType = treeKey.toUpperCase() as PageType;
        if (treeKey === 'vertices' || treeKey === 'edges') {
            const projectPath: string = store.state.openedProject.path;
            const tablePath = await join(projectPath, pageType.toLowerCase());
            const data: object = await openTable(tablePath);

            panes.value.push({ title: treeKey, key: treeKey, closable: true, type: pageType, data: data });
            activeKey.value = treeKey;

        };
    }

};


</script>
<style>
.ant-tabs-top>.ant-tabs-nav {
    margin: 0px;
}

.ant-tabs-content-holder {
    width: 100%;
    height: 100%;
}

.ant-tabs {
    width: 100%;
    height: 100%;
}

.ant-tabs-content {
    width: 100%;
    height: 100%;
}

#components-layout-demo-side .logo {
    height: 32px;
    margin: 16px;
    background: rgba(255, 255, 255, 0.3);
}

.site-layout .site-layout-background {
    background: #fff;
}

[data-theme='dark'] .site-layout .site-layout-background {
    background: #141414;
}

#view {
    background: #fff;
}
</style>
